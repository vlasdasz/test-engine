use std::{cell::RefCell, collections::HashMap, f64, mem::size_of};

use anyhow::Result;
use bytemuck::cast_slice;
use gm::{CheckedConvert, Color, LossyConvert, Platform, U8Color, flat::Size};
use tokio::{
    spawn,
    sync::oneshot::{Receiver, Sender, channel},
};
use wgpu::{Buffer, BufferDescriptor, COPY_BYTES_PER_ROW_ALIGNMENT, CommandEncoder, Extent3d, TextureFormat};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop};

use crate::{
    SUPPORT_SCREENSHOT, Screenshot, Window, app::App, frame_counter::FrameCounter, image::Texture, text::Font,
};

type ReadDisplayRequest = Sender<Screenshot>;

#[cfg(not(target_os = "android"))]
pub(crate) const RGBA_TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
#[cfg(target_os = "android")]
pub(crate) const RGBA_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

pub struct State {
    pub(crate) fonts: HashMap<&'static str, Font>,
    pub(crate) app:   Box<dyn App>,

    read_display_request: RefCell<Option<ReadDisplayRequest>>,

    pub(crate) frame_counter: FrameCounter,
}

impl State {
    pub fn new(app: Box<dyn App>) -> Self {
        Self {
            fonts: HashMap::default(),
            app,
            read_display_request: RefCell::default(),
            frame_counter: FrameCounter::default(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, event_loop: &ActiveEventLoop) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        let window = Window::current();

        window.size = (new_size.width, new_size.height).into();
        window.config.width = new_size.width;
        window.config.height = new_size.height;

        if let Some(surface) = &mut window.surface {
            surface.depth_texture = Texture::create_depth_texture(
                &window.device,
                (new_size.width, new_size.height).into(),
                "depth_texture",
            );
            surface.presentable.configure(&window.device, &window.config);
        } else if window.resumed && window.create_surface_and_window(event_loop).unwrap() {
            window.state.app.window_ready();
        }

        let queue = Window::queue();

        for font in self.fonts.values() {
            font.brush.resize_view(
                window.config.width.lossy_convert(),
                window.config.height.lossy_convert(),
                queue,
            );
        }

        let inner_size = Window::inner_size();

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

    pub fn update(&mut self) {
        self.app.update();

        if self.frame_counter.update() {
            let a = format!(
                "{:.2}ms frame {:.1} FPS",
                self.frame_counter.frame_time * 1000.0,
                self.frame_counter.fps
            );
            let app = Window::current();
            Window::current().set_title(format!("{a} {} x {}", app.config.width, app.config.height));
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let app = Window::current();
        let Some(ref surface) = app.surface else {
            return Ok(());
        };
        let surface_texture = surface.presentable.get_current_texture()?;
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = Window::device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                app.config.width.lossy_convert(),
                app.config.height.lossy_convert(),
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

        Window::queue().submit(std::iter::once(encoder.finish()));
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

        let (s, r) = channel();
        request.replace(s);
        r
    }

    fn read_screen(encoder: &mut CommandEncoder, texture: &wgpu::Texture) -> (Buffer, Size<u32>) {
        if !SUPPORT_SCREENSHOT {
            return (
                Window::device().create_buffer(&BufferDescriptor {
                    label:              Some("Empty Buffer"),
                    size:               0,
                    usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                Size::default(),
            );
        }

        let screen_width_bytes: u64 = u64::from(texture.size().width) * size_of::<u32>() as u64;

        let number_of_align = screen_width_bytes / u64::from(COPY_BYTES_PER_ROW_ALIGNMENT) + 1;

        let width_bytes = number_of_align * u64::from(COPY_BYTES_PER_ROW_ALIGNMENT);

        let buffer = Window::device().create_buffer(&BufferDescriptor {
            label:              Some("Read Screen Buffer"),
            size:               width_bytes * u64::from(texture.size().height),
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
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
