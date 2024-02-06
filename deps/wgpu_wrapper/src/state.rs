use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use wgpu::{CompositeAlphaMode, PresentMode, TextureFormat};
use winit::{event::WindowEvent, window::Window};

use crate::{app::App, text::Font, wgpu_drawer::WGPUDrawer};

pub struct State {
    surface:           wgpu::Surface<'static>,
    pub(crate) config: wgpu::SurfaceConfiguration,

    pub(crate) drawer: WGPUDrawer,

    pub(crate) fonts: HashMap<&'static str, Font>,
    pub(crate) app:   Box<dyn App>,
}

impl State {
    pub async fn new(app: impl App + 'static, window: Arc<Window>) -> Result<Self> {
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

        dbg!(surface_caps);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:       TextureFormat::Bgra8UnormSrgb,
            width:        size.width,
            height:       size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode:   CompositeAlphaMode::Auto,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let drawer = WGPUDrawer::new(device, queue, config.format)?;

        Ok(Self {
            surface,
            config,
            drawer,
            fonts: Default::default(),
            app: Box::new(app),
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
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

    pub fn _input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        self.app.update();
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

        self.drawer.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
