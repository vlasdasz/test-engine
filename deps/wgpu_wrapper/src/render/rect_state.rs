use std::ops::Range;

use bytemuck::cast_slice;
use gm::{
    checked_usize_to_u32,
    flat::{Point, Rect},
    Color,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, RenderPass, RenderPipeline,
    TextureFormat,
};

use crate::{
    render::{new_uniform::Uniform, uniform::OldUniform},
    utils::make_pipeline,
    WGPUApp,
};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct RectState {
    z_layout:      BindGroupLayout,
    pipeline:      RenderPipeline,
    vertex_buffer: Buffer,
}

impl RectState {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let z_layout = OldUniform::z_layout();

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&z_layout, Color::layout()],
            push_constant_ranges: &[],
        });

        let pipeline = make_pipeline::<Point>(
            "Rect Fill Render Pipeline",
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            z_layout,
            pipeline,
            vertex_buffer,
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>, rect: &Rect, color: &Color, z_position: f32) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, OldUniform::z(&self.z_layout, z_position), &[]);
        render_pass.set_bind_group(1, Color::bind(*color), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(VERTEX_RANGE, 0..1);
    }
}
