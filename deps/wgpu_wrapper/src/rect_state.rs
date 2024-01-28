use std::collections::HashMap;

use bytemuck::cast_slice;
use gm::{
    flat::{Point, Rect},
    Color,
};
use refs::MainLock;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
    BufferBinding, BufferBindingType, BufferUsages, Device, PipelineLayoutDescriptor, RenderPass,
    RenderPipeline, ShaderStages, TextureFormat,
};

use crate::utils::make_pipeline;

static BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();

#[derive(Debug)]
pub struct RectState {
    bind_group_layout:   BindGroupLayout,
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer:   Buffer,
    pub num_vertices:    u32,
}

impl RectState {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        const RECT_VERTICES: &[Point] = &[
            Point::new(-1.0, 1.0),
            Point::new(-1.0, -1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, -1.0),
        ];

        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("rect_bind_group_layout"),
            entries: &[BindGroupLayoutEntry {
                binding:    0,
                visibility: ShaderStages::FRAGMENT,
                ty:         BindingType::Buffer {
                    ty:                 BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count:      None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<Point>(
            "Rect Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(RECT_VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            bind_group_layout,
            render_pipeline,
            vertex_buffer,
            num_vertices: u32::try_from(RECT_VERTICES.len()).unwrap(),
        }
    }

    fn bind_group_with_color(
        bind_group_layout: &BindGroupLayout,
        device: &Device,
        color: &Color,
    ) -> BindGroup {
        let color_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Color Uniform Buffer"),
            contents: cast_slice(&color.as_slice()),
            usage:    BufferUsages::UNIFORM,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("rect_bind_group"),
            layout:  bind_group_layout,
            entries: &[BindGroupEntry {
                binding:  0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            }],
        })
    }

    pub fn draw<'a>(&'a self, device: &Device, render_pass: &mut RenderPass<'a>, rect: &Rect, color: &Color) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);

        let bind = BINDS
            .get_mut()
            .entry(*color)
            .or_insert_with(|| Self::bind_group_with_color(&self.bind_group_layout, device, color));

        render_pass.set_bind_group(0, bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
