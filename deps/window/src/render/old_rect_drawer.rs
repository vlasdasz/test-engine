use std::ops::Range;

use gm::{
    Color, checked_usize_to_u32,
    flat::{Point, Rect},
};
use wgpu::{
    BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPipeline, ShaderStages, include_wgsl,
};

use crate::{
    Window,
    render::{
        uniform::{cached_color_bind, cached_float_bind, make_uniform_layout},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct OldRectDrawer {
    pipeline:        RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
    fragment_layout: BindGroupLayout,
}

impl Default for OldRectDrawer {
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let vertex_layout = make_uniform_layout("old_rect_vertex_layout", ShaderStages::VERTEX);
        let fragment_layout = make_uniform_layout("old_rect_vertex_layout", ShaderStages::FRAGMENT);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Old Rect Pipeline Layout"),
            bind_group_layouts:   &[&vertex_layout, &fragment_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.pipeline(
            "old_rect_fill_pipeline",
            &pipeline_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            vertex_buffer: device.buffer(VERTICES, BufferUsages::VERTEX),
            vertex_layout,
            fragment_layout,
        }
    }
}

impl OldRectDrawer {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>, rect: &Rect, color: &Color, z_position: f32) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0., 1.);
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, cached_float_bind(z_position, &self.vertex_layout), &[]);
        render_pass.set_bind_group(1, cached_color_bind(*color, &self.fragment_layout), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(VERTEX_RANGE, 0..1);
    }
}
