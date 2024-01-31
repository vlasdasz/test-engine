use std::sync::Arc;

use anyhow::{anyhow, Result};
use gm::Color;
use manage::data_manager::DataManager;
use wgpu::{CompositeAlphaMode, PresentMode, TextureFormat};
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, BuiltInLineBreaker, HorizontalAlign, Layout, VerticalAlign},
    BrushBuilder, TextBrush,
};
use winit::{event::WindowEvent, window::Window};

use crate::{image::Image, wgpu_drawer::WGPUDrawer};

pub struct State {
    surface:           wgpu::Surface<'static>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue:  wgpu::Queue,
    config:            wgpu::SurfaceConfiguration,

    drawer: WGPUDrawer,

    pub size: winit::dpi::PhysicalSize<u32>,

    pub brush: TextBrush<FontRef<'static>>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
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

        //    dbg!(&surface_caps);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a
        // different one will result in all the colors coming out darker. If you
        // want to support non sRGB surfaces, you'll need to account for that
        // when drawing to the frame.
        // let surface_format = surface_caps
        //     .formats
        //     .iter()
        //     .copied()
        //     .find(TextureFormat::is_srgb)
        //     .unwrap_or(surface_caps.formats[0]);
        //
        // dbg!(&surface_format);

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

        let drawer = WGPUDrawer::new(&device, config.format)?;

        let brush = BrushBuilder::using_font_bytes(include_bytes!("text/fonts/Helvetica.ttf")).unwrap()
            /* .initial_cache_size((16_384, 16_384))) */ // use this to avoid resizing cache texture
            .build(&device, config.width, config.height, config.format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            drawer,
            size,
            brush,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.brush
                .resize_view(self.config.width as f32, self.config.height as f32, &self.queue);
        }
    }

    pub fn _input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        use wgpu_text::glyph_brush::{Section as TextSection, Text};

        let section = TextSection::default()
            .add_text(
                Text::new("Hello World Плати ęčėčė0- Ye ЩООООФФ")
                    .with_scale(100.0)
                    .with_color(Color::BLACK.as_slice()),
            )
            .with_bounds((self.config.width as f32 * 0.4, self.config.height as f32))
            .with_layout(
                Layout::default()
                    .v_align(VerticalAlign::Center)
                    .h_align(HorizontalAlign::Center)
                    .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
            )
            .with_screen_position((400.0, self.config.height as f32 * 0.5));

        self.brush.queue(&self.device, &self.queue, vec![&section]).unwrap();

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

            //
            // let image = Image::from_texture(texture, &self.device)?;
            //
            // let image = Image::add_with_name("azaza_text", || image);
            //
            // self.drawer
            //     .image_state
            //     .draw(image.get_static(), &(500, 500, 200, 200).into(), &mut
            // render_pass);

            // let manual = Image::add_with_name("manual_image", || {
            //     let font = Font::helvetice().unwrap();
            //     let texture = font.draw(&self.device, &self.queue, "ЫЖЦДЙ
            // §-ž9ę-č0ė9").unwrap();     Image::from_texture(texture,
            // &self.device).unwrap() });
            //
            // self.drawer.colored_image_state.draw(
            //     manual.get_static(),
            //     &(280, 500, 800, 200).into(),
            //     &mut render_pass,
            // );

            self.drawer.colored_image_state.draw(
                Image::get("happy-tree.png").get_static(),
                &(10, 500, 200, 200).into(),
                &mut render_pass,
            );

            self.drawer.colored_image_state.draw(
                Image::get("frisk.png").get_static(),
                &(400, 10, 100, 100).into(),
                &mut render_pass,
            );

            self.drawer.fill_rect(
                &self.device,
                &mut render_pass,
                &(10, 10, 100, 100).into(),
                &Color::GREEN,
                1,
            );

            self.drawer.fill_rect(
                &self.device,
                &mut render_pass,
                &(400, 200, 200, 200).into(),
                &Color::BLUE,
                1,
            );

            render_pass.set_viewport(
                0.0,
                0.0,
                self.config.width as f32,
                self.config.height as f32,
                0.0,
                1.0,
            );

            self.brush.draw(&mut render_pass);

            // Crashes if inner cache exceeds limits.
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
