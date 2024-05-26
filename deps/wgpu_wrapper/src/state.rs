use std::{cell::RefCell, collections::HashMap, f64, mem::size_of, sync::Arc};

use anyhow::Result;
use bytemuck::cast_slice;
use gm::{flat::Size, CheckedConvert, Color, LossyConvert, Platform, U8Color};
use refs::MainLock;
use tokio::{
    spawn,
    sync::oneshot::{channel, Receiver, Sender},
};
use wgpu::{Buffer, BufferDescriptor, CommandEncoder, Extent3d, TextureFormat, COPY_BYTES_PER_ROW_ALIGNMENT};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    app::App, frame_counter::FrameCounter, image::Texture, render::wgpu_drawer::WGPUDrawer, surface::Surface,
    text::Font, Screenshot, WGPUApp,
};

type ReadDisplayRequest = Sender<Screenshot>;

pub(crate) static SURFACE: MainLock<Option<Surface>> = MainLock::new();

pub(crate) static DRAWER: MainLock<Option<WGPUDrawer>> = MainLock::new();

pub(crate) const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;

pub struct State {
    pub(crate) window: Arc<Window>,

    pub(crate) fonts: HashMap<&'static str, Font>,
    pub(crate) app:   Box<dyn App>,

    pub(crate) fps:        f32,
    pub(crate) frame_time: f32,

    read_display_request: RefCell<Option<ReadDisplayRequest>>,

    frame_counter: FrameCounter,
}

impl State {
    pub async fn new(app: Box<dyn App>, window: Arc<Window>) -> Result<Self> {
        *SURFACE.get_mut() = Surface::new(window.clone()).await?.into();
        *DRAWER.get_mut() = WGPUDrawer::new()?.into();

        Ok(Self {
            window,
            fonts: Default::default(),
            app,
            fps: 0.0,
            frame_time: 0.0,
            read_display_request: Default::default(),
            frame_counter: Default::default(),
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let surface = WGPUApp::surface_mut();
            surface.depth_texture = Texture::create_depth_texture(
                &surface.device,
                (new_size.width, new_size.height).into(),
                "depth_texture",
            );
            WGPUApp::drawer().window_size = (new_size.width, new_size.height).into();
            surface.config.width = new_size.width;
            surface.config.height = new_size.height;
            surface.presentable.configure(WGPUApp::device(), &surface.config);

            let queue = WGPUApp::queue();

            for font in self.fonts.values() {
                font.brush.resize_view(
                    surface.config.width.lossy_convert(),
                    surface.config.height.lossy_convert(),
                    queue,
                );
            }

            let inner_size = self.window.inner_size();

            let position = if Platform::IOS {
                // match self.window.inner_position() {
                //     Ok(pos) => (pos.x, pos.y),
                //     Err(err) => {
                //         error!("{err}");
                (0, 0)
                //      }
                //    }
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
            let a = format!("{:.2}ms frame {fps:.1} FPS", frame_time * 1000.0);
            self.fps = fps;
            self.frame_time = frame_time;
            let surface = WGPUApp::surface();
            WGPUApp::current().set_title(format!(
                "{a} {} x {}",
                surface.config.width, surface.config.height
            ))
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let surface = WGPUApp::surface();
        let surface_texture = surface.presentable.get_current_texture()?;
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
                    view:        &surface.depth_texture.view,
                    depth_ops:   Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set:      None,
                timestamp_writes:         None,
            });

            self.app.render(&mut render_pass);

            render_pass.set_viewport(
                0.0,
                0.0,
                surface.config.width.lossy_convert(),
                surface.config.height.lossy_convert(),
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
