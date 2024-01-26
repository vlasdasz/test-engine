use gm::{flat::Point, Color};
use wgpu::util::DeviceExt;

use crate::VertexLayout;

pub struct RectState {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group:      wgpu::BindGroup,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl RectState {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/rect.wgsl"));

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
            label:   Some("rect_bind_group"),
            layout:  &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            }],
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

        Self {
            render_pipeline,
            bind_group,
            vertex_buffer,
            num_vertices: RECT_VERTICES.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(200.0, 200.0, 200.0, 200.0, 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
