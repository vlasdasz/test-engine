use anyhow::Result;
use gm::{flat::Point, volume::UIVertex};
use wgpu::util::DeviceExt;

use crate::{texture::Texture, VertexLayout};

pub struct ImageState {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group:      wgpu::BindGroup,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl ImageState {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

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
            label:   Some("image_bind_group_layout"),
        });

        let diffuse_bytes = include_bytes!("../../Assets/Images/happy-tree.png");
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png")?;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                entry_point: "v_main",                     // 1.
                buffers:     &[UIVertex::vertex_layout()], // 2.
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
                topology:           wgpu::PrimitiveTopology::TriangleStrip, // 1.
                strip_index_format: None,
                front_face:         wgpu::FrontFace::Ccw, // 2.
                cull_mode:          Some(wgpu::Face::Back),
                polygon_mode:       wgpu::PolygonMode::Fill,
                unclipped_depth:    false,
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

        const VERTICES: &[UIVertex] = &[
            UIVertex {
                pos: Point::new(-1.0, 1.0),
                uv:  Point::new(0.0, 0.0),
            },
            UIVertex {
                pos: Point::new(-1.0, -1.0),
                uv:  Point::new(0.0, 1.0),
            },
            UIVertex {
                pos: Point::new(1.0, 1.0),
                uv:  Point::new(1.0, 0.0),
            },
            UIVertex {
                pos: Point::new(1.0, -1.0),
                uv:  Point::new(1.0, 1.0),
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        Ok(Self {
            render_pipeline,
            bind_group,
            vertex_buffer,
            num_vertices,
        })
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(10.0, 10.0, 1000.0, 1000.0, 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
