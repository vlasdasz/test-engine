// lib.rs
use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::{texture::Texture, Vertex, INDICES, VERTICES};

pub struct State {
    surface: wgpu::Surface<'static>,
    device:  wgpu::Device,
    queue:   wgpu::Queue,
    config:  wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,

    index_buffer: wgpu::Buffer,
    num_indices:  u32,

    diffuse_bind_group: wgpu::BindGroup, // NEW!

    pub size: winit::dpi::PhysicalSize<u32>,

    _diffuse_texture: Texture, // NEW
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface = instance.create_surface(window.clone()).unwrap();

        let _adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::default(),
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .into_iter()
            .filter(|adapter| {
                // Check if this adapter supports our surface
                adapter.is_surface_supported(&surface)
            })
            .next()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits:   if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label:             None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a
        // different one will result in all the colors coming out darker. If you
        // want to support non sRGB surfaces, you'll need to account for that
        // when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:       surface_format,
            width:        size.width,
            height:       size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode:   surface_caps.alpha_modes[0],
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let _modes = &surface_caps.present_modes;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding:    0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty:         wgpu::BindingType::Texture {
                        multisampled:   false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type:    wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count:      None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding:    1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty:         wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count:      None,
                },
            ],
            label:   Some("texture_bind_group_layout"),
        });

        surface.configure(&device, &config);
        let diffuse_bytes = include_bytes!("../../Assets/Images/happy-tree.png");
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        // Everything up until `let texture_bind_group_layout = ...` can now be removed.

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
                },
                wgpu::BindGroupEntry {
                    binding:  1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
                },
            ],
            label:   Some("diffuse_bind_group"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Render Pipeline Layout"),
            bind_group_layouts:   &[&texture_bind_group_layout], // NEW!
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:         Some("Render Pipeline"),
            layout:        Some(&render_pipeline_layout),
            vertex:        wgpu::VertexState {
                module:      &shader,
                entry_point: "vs_main",         // 1.
                buffers:     &[Vertex::desc()], // 2.
            },
            fragment:      Some(wgpu::FragmentState {
                // 3.
                module:      &shader,
                entry_point: "fs_main",
                targets:     &[Some(wgpu::ColorTargetState {
                    // 4.
                    format:     config.format,
                    blend:      Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive:     wgpu::PrimitiveState {
                topology:           wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face:         wgpu::FrontFace::Ccw, // 2.
                cull_mode:          Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode:       wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth:    false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative:       false,
            },
            depth_stencil: None, // 1.
            multisample:   wgpu::MultisampleState {
                count:                     1,     // 2.
                mask:                      !0,    // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview:     None, // 5.
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage:    wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            _diffuse_texture: diffuse_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn _input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            // render()
            // render()
            // ...
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
