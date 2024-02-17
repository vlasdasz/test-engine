use std::{cell::RefCell, collections::HashMap, mem::size_of, sync::Arc};

use anyhow::{anyhow, Result};
use bytemuck::cast_slice;
use gm::{flat::Size, U8Color};
use tokio::{
    spawn,
    sync::oneshot::{channel, Receiver, Sender},
};
use wgpu::{
    Buffer, BufferDescriptor, CommandEncoder, CompositeAlphaMode, Extent3d, PresentMode, Texture,
    TextureFormat, COPY_BYTES_PER_ROW_ALIGNMENT,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{app::App, frame_counter::FrameCounter, render::wgpu_drawer::WGPUDrawer, text::Font};

type ReadDisplayRequest = Sender<(Vec<U8Color>, Size<u32>)>;

pub struct State {
    surface:           wgpu::Surface<'static>,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) window: Arc<Window>,

    pub drawer: WGPUDrawer,

    pub(crate) fonts: HashMap<&'static str, Font>,
    pub(crate) app:   Box<dyn App>,

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

        let surface_caps = surface.get_capabilities(&adapter);

        dbg!(&surface_caps);

        let alpha_mode = if surface_caps.alpha_modes.contains(&CompositeAlphaMode::PostMultiplied) {
            CompositeAlphaMode::PostMultiplied
        } else {
            CompositeAlphaMode::Auto
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let drawer = WGPUDrawer::new(device, queue, config.format)?;

        Ok(Self {
            surface,
            config,
            window,
            drawer,
            fonts: Default::default(),
            app,
            read_display_request: Default::default(),
            frame_counter: Default::default(),
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.drawer.window_size = (new_size.width, new_size.height).into();
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.drawer.device, &self.config);
            for font in self.fonts.values() {
                font.brush.resize_view(
                    self.config.width as f32,
                    self.config.height as f32,
                    &self.drawer.queue,
                );
            }
            self.app.resize((new_size.width, new_size.height).into());
        }
    }

    pub fn update(&mut self) {
        self.app.update();
        if let Some(fps) = self.frame_counter.update() {
            self.window
                .set_title(&format!("{fps} {} x {}", self.config.width, self.config.height));
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.drawer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
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
            Some(self.read_screen(&mut encoder, &surface_texture.texture))
        } else {
            None
        };

        self.drawer.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        if let Some(buffer_sender) = self.read_display_request.take() {
            let (sender, receiver) = channel();

            let Some(buffer) = buffer else { return Ok(()) };

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

                buffer_sender.send((data, size)).unwrap();
            });
        }

        Ok(())
    }

    pub fn request_read_display(&self) -> Receiver<(Vec<U8Color>, Size<u32>)> {
        let (s, r) = channel();
        self.read_display_request.replace(s.into());
        r
    }

    fn read_screen(&self, encoder: &mut CommandEncoder, texture: &Texture) -> (Buffer, Size<u32>) {
        let screen_width_bytes: u64 = u64::from(texture.size().width) * size_of::<u32>() as u64;

        let number_of_align = screen_width_bytes / u64::from(COPY_BYTES_PER_ROW_ALIGNMENT) + 1;

        let width_bytes = number_of_align * u64::from(COPY_BYTES_PER_ROW_ALIGNMENT);

        let buffer = self.drawer.device.create_buffer(&BufferDescriptor {
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
