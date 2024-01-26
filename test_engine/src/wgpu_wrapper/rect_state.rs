use gm::{
    flat::{Point, Rect},
    Color,
};
use wgpu::util::DeviceExt;

use crate::wgpu_wrapper::utils::make_pipeline;

#[derive(Debug)]
pub struct RectState {
    bind_group:          wgpu::BindGroup,
    bind_group_layout:   wgpu::BindGroupLayout,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl RectState {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/rect.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("rect_bind_group_layout"),
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
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<Point>(
            "Rect Render Pipeline",
            &device,
            &pipeline_layout,
            &shader,
            texture_format,
        );

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

        let bind_group = Self::bind_group_with_color(&bind_group_layout, device, &Color::TURQUOISE);

        Self {
            bind_group: bind_group.into(),
            bind_group_layout,
            render_pipeline,
            vertex_buffer,
            num_vertices: RECT_VERTICES.len() as u32,
        }
    }

    fn bind_group_with_color(
        bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        color: &Color,
    ) -> wgpu::BindGroup {
        let color_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Color Uniform Buffer"),
            contents: bytemuck::cast_slice(&color.as_slice()),
            usage:    wgpu::BufferUsages::UNIFORM,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("rect_bind_group"),
            layout:  bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            }],
        })
    }

    pub fn draw<'a>(
        &'a self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'a>,
        rect: &Rect,
        color: &Color,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
