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
    BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        uniform::{make_bind, make_uniform_layout},
        vertex_layout::VertexLayout,
    },
    utils::make_pipeline,
    WGPUApp,
};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

fn gen_vertices(index: u32) -> Point {
    (index & 1, (index & 2) >> 1).into()
}

#[test]
fn test_gen() {
    for i in 0..6 {
        dbg!(gen_vertices(i));
    }
}

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct RectDrawer {
    pipeline:        RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
    fragment_layout: BindGroupLayout,
}

impl RectDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let vertex_layout = make_uniform_layout("rect_vertext_layout", ShaderStages::VERTEX);
        let fragment_layout = make_uniform_layout("rect_vertext_layout", ShaderStages::FRAGMENT);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&vertex_layout, &fragment_layout],
            push_constant_ranges: &[],
        });

        let pipeline = make_pipeline(
            "Rect Fill Render Pipeline",
            Some(&pipeline_layout),
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT],
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertex_layout,
            fragment_layout,
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>, rect: &Rect, color: &Color, z_position: f32) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0., 1.);
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, make_bind(&z_position, &self.vertex_layout), &[]);
        render_pass.set_bind_group(1, make_bind(color, &self.fragment_layout), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(VERTEX_RANGE, 0..1);
    }
}
