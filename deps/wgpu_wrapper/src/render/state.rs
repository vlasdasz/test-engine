use std::{cell::RefCell, collections::HashMap, f64, mem::size_of, sync::Arc};

use anyhow::{anyhow, Result};
use bytemuck::cast_slice;
use gm::{flat::Size, CheckedConvert, Color, Platform, U8Color};
use log::error;
use refs::MainLock;
use tokio::{
    spawn,
    sync::oneshot::{channel, Receiver, Sender},
};
use wgpu::{
    Buffer, BufferDescriptor, CommandEncoder, CompositeAlphaMode, Device, Extent3d, PresentMode, Queue,
    TextureFormat, COPY_BYTES_PER_ROW_ALIGNMENT,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    app::App, frame_counter::FrameCounter, image::Texture, render::wgpu_drawer::WGPUDrawer, text::Font,
    Screenshot, WGPUApp,
};

type ReadDisplayRequest = Sender<Screenshot>;

pub(crate) static DEVICE: MainLock<Option<Device>> = MainLock::const_new();
pub(crate) static QUEUE: MainLock<Option<Queue>> = MainLock::const_new();

pub struct State {
    surface:           wgpu::Surface<'static>,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) window: Arc<Window>,

    pub(crate) depth_texture: Texture,

    pub drawer: WGPUDrawer,

    pub(crate) fonts: HashMap<&'static str, Font>,
    pub(crate) app:   Box<dyn App>,

    pub(crate) fps: f32,

    read_display_request: RefCell<Option<ReadDisplayRequest>>,

    frame_counter: FrameCounter,
}

impl State {
    pub async fn new(app: Box<dyn App>, window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or(anyhow!("Failed to request adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::POLYGON_MODE_LINE,
                    required_limits:   if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label:             None,
                },
                None,
            )
            .await?;

        let _surface_caps = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format:       TextureFormat::Bgra8UnormSrgb,
            width:        size.width,
            height:       size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode:   CompositeAlphaMode::Auto,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        *DEVICE.get_mut() = device.into();
        *QUEUE.get_mut() = queue.into();

        let depth_texture =
            Texture::create_depth_texture((config.width, config.height).into(), "depth_texture");

        let drawer = WGPUDrawer::new(config.format)?;

        Ok(Self {
            surface,
            config,
            window,
            depth_texture,
            drawer,
            fonts: Default::default(),
            app,
            fps: 0.0,
            read_display_request: Default::default(),
            frame_counter: Default::default(),
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.depth_texture =
                Texture::create_depth_texture((new_size.width, new_size.height).into(), "depth_texture");
            self.drawer.window_size = (new_size.width, new_size.height).into();
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(WGPUApp::device(), &self.config);

            let queue = WGPUApp::queue();

            for font in self.fonts.values() {
                font.brush
                    .resize_view(self.config.width as f32, self.config.height as f32, queue);
            }

            let inner_size = self.window.inner_size();

            let position = if Platform::IOS {
                match self.window.inner_position() {
                    Ok(pos) => (pos.x, pos.y),
                    Err(err) => {
                        error!("{err}");
                        (0, 0)
                    }
                }
            } else {
                (0, 0)
            };

            self.app.resize(
                position.into(),
                (inner_size.width, inner_size.height - position.1.checked_convert()).into(),
            );
        }
    }

    pub fn update(&mut self) {
        self.app.update();
        if let Some((frame_time, fps)) = self.frame_counter.update() {
            let a = format!("{frame_time:.2}ms frame {fps:.1} FPS");
            self.fps = fps;
            self.window
                .set_title(&format!("{a} {} x {}", self.config.width, self.config.height));
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = WGPUApp::device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label:                    Some("Render Pass"),
                color_attachments:        &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    ops:            wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color {
                            r: f64::from(Color::GRAY_BLUE.r),
                            g: f64::from(Color::GRAY_BLUE.g),
                            b: f64::from(Color::GRAY_BLUE.b),
                            a: f64::from(Color::GRAY_BLUE.a),
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view:        &self.depth_texture.view,
                    depth_ops:   Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set:      None,
                timestamp_writes:         None,
            });

            self.app.render(&mut render_pass, &self.drawer);

            render_pass.set_viewport(
                0.0,
                0.0,
                self.config.width as f32,
                self.config.height as f32,
                0.0,
                1.0,
            );

            for font in self.fonts.values() {
                font.brush.draw(&mut render_pass);
            }
        }

        let buffer = if self.read_display_request.borrow().is_some() {
            Some(Self::read_screen(&mut encoder, &surface_texture.texture))
        } else {
            None
        };

        WGPUApp::queue().submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        if let Some(buffer_sender) = self.read_display_request.take() {
            let (sender, receiver) = channel();

            let Some(buffer) = buffer else {
                return Ok(());
            };

            let buffer_slice = buffer.0.slice(..);

            buffer_slice.map_async(wgpu::MapMode::Read, |result| {
                sender.send(result).unwrap();
            });

            spawn(async move {
                let _ = receiver.await.unwrap();
                let (buff, size) = buffer;

                let bytes: &[u8] = &buff.slice(..).get_mapped_range();
                let data: Vec<U8Color> =
                    cast_slice(bytes).iter().map(|color: &U8Color| color.bgra_to_rgba()).collect();

                buffer_sender.send(Screenshot::new(data, size)).unwrap();
            });
        }

        Ok(())
    }

    pub fn request_read_display(&self) -> Receiver<Screenshot> {
        let mut request = self.read_display_request.borrow_mut();

        //  assert!(request.is_none());

        let (s, r) = channel();
        request.replace(s);
        r
    }

    fn read_screen(encoder: &mut CommandEncoder, texture: &wgpu::Texture) -> (Buffer, Size<u32>) {
        let screen_width_bytes: u64 = u64::from(texture.size().width) * size_of::<u32>() as u64;

        let number_of_align = screen_width_bytes / u64::from(COPY_BYTES_PER_ROW_ALIGNMENT) + 1;

        let width_bytes = number_of_align * u64::from(COPY_BYTES_PER_ROW_ALIGNMENT);

        let buffer = WGPUApp::device().create_buffer(&BufferDescriptor {
            label:              Some("Read Screen Buffer"),
            size:               width_bytes * u64::from(texture.size().height),
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset:         0,
                    bytes_per_row:  u32::try_from(width_bytes).unwrap().into(),
                    rows_per_image: texture.size().height.into(),
                },
            },
            Extent3d {
                width:                 texture.size().width,
                height:                texture.size().height,
                depth_or_array_layers: 1,
            },
        );

        let size: Size<u32> = Size::new(
            u32::try_from(width_bytes / size_of::<U8Color>() as u64).unwrap(),
            texture.size().height,
        );

        (buffer, size)
    }
}
