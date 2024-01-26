use std::sync::Arc;

use anyhow::{anyhow, Result};
use gm::{flat::Point, Color};
use wgpu::{util::DeviceExt, CompositeAlphaMode, PresentMode};
use winit::{event::WindowEvent, window::Window};

use crate::{texture::Texture, Vertex, VertexLayout, INDICES, VERTICES};

struct RectState {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer:   wgpu::Buffer,
    bind_group:      wgpu::BindGroup,
    num_vertices:    u32,
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device:  wgpu::Device,
    queue:   wgpu::Queue,
    config:  wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,

    index_buffer: wgpu::Buffer,
    num_indices:  u32,

    diffuse_bind_group: wgpu::BindGroup,

    pub size: winit::dpi::PhysicalSize<u32>,

    _diffuse_texture: Texture,

    rect_state: RectState,
}

impl State {
    async fn make_rect_state(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> RectState {
        let shader = device.create_shader_module(wgpu::include_wgsl!("rect.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty:         wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count:      None,
            }],
            label:   Some("rect_bind_group_layout"),
        });

        let color_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Color Uniform Buffer"),
            contents: bytemuck::cast_slice(&Color::TURQUOISE.as_slice()),
            usage:    wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            }],
            label:   Some("rect_bind_group"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:         Some("Rect Render Pipeline"),
            layout:        Some(&render_pipeline_layout),
            vertex:        wgpu::VertexState {
                module:      &shader,
                entry_point: "v_main",                  // 1.
                buffers:     &[Point::vertex_layout()], // 2.
            },
            fragment:      Some(wgpu::FragmentState {
                // 3.
                module:      &shader,
                entry_point: "f_main",
                targets:     &[Some(wgpu::ColorTargetState {
                    // 4.
                    format:     texture_format,
                    blend:      Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive:     wgpu::PrimitiveState {
                topology:           wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face:         wgpu::FrontFace::Ccw,
                cull_mode:          Some(wgpu::Face::Back),
                polygon_mode:       wgpu::PolygonMode::Fill,
                unclipped_depth:    false,
                conservative:       false,
            },
            depth_stencil: None,
            multisample:   wgpu::MultisampleState {
                count:                     1,
                mask:                      !0,
                alpha_to_coverage_enabled: false,
            },
            multiview:     None,
        });

        const RECT_VERTICES: &[Point] = &[
            Point::new(-1.0, 1.0),
            Point::new(-1.0, -1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, -1.0),
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(RECT_VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        RectState {
            render_pipeline,
            vertex_buffer,
            bind_group,
            num_vertices: RECT_VERTICES.len() as u32,
        }
    }

    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();

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
            present_mode: PresentMode::AutoVsync,
            alpha_mode:   CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

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

        let diffuse_bytes = include_bytes!("../../Assets/Images/happy-tree.png");
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png")?;

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
            label:                Some("Image Render Pipeline Layout"),
            bind_group_layouts:   &[&texture_bind_group_layout], // NEW!
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:         Some("Image Render Pipeline"),
            layout:        Some(&render_pipeline_layout),
            vertex:        wgpu::VertexState {
                module:      &shader,
                entry_point: "v_main",          // 1.
                buffers:     &[Vertex::desc()], // 2.
            },
            fragment:      Some(wgpu::FragmentState {
                // 3.
                module:      &shader,
                entry_point: "f_main",
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
            label:    Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Image Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage:    wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let rect_state = Self::make_rect_state(&device, config.format).await;

        Ok(Self {
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
            rect_state,
        })
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
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            render_pass.set_viewport(200.0, 200.0, 200.0, 200.0, 0.0, 1.0);
            render_pass.set_pipeline(&self.rect_state.render_pipeline);
            render_pass.set_bind_group(0, &self.rect_state.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.rect_state.vertex_buffer.slice(..));
            render_pass.draw(0..self.rect_state.num_vertices, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
