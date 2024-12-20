use std::ops::Range;

use gm::{
    Color, checked_usize_to_u32,
    flat::{Point, Rect, Size},
};
use wgpu::{
    Buffer, PipelineLayoutDescriptor, PrimitiveTopology, RenderPass, RenderPipeline, ShaderStages,
    include_wgsl,
};

use crate::{
    BufferUsages, PolygonMode, Window,
    render::{
        flat::{rect_instance::RectInstance, rect_view::RectView},
        uniform::{UniformBind, make_uniform_layout},
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
};

const RECT_VERTICERS: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const RECT_VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(RECT_VERTICERS.len());

#[derive(Debug)]
pub struct RectPipeline {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    view:  UniformBind<RectView>,
    rects: VecBuffer<RectInstance>,
}

impl Default for RectPipeline {
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(include_wgsl!("rect.wgsl"));

        let sprite_view_layout = make_uniform_layout("rect_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "rect_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.pipeline(
            "rect_drawer_pipeline",
            &uniform_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, RectInstance::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            view: sprite_view_layout.into(),
            vertex_buffer: device.buffer(RECT_VERTICERS, BufferUsages::VERTEX),
            rects: VecBuffer::default(),
        }
    }
}

impl RectPipeline {
    pub fn add(&mut self, rect: Rect, color: Color, z_position: f32) {
        self.rects.push(RectInstance {
            origin: rect.origin,
            size: rect.size,
            color,
            z_position,
            _padding: 0,
        });
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, resolution: Size) {
        if self.rects.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);

        self.view.update(RectView { resolution });

        self.rects.load();

        render_pass.set_bind_group(0, self.view.bind(), &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.rects.buffer().slice(..));

        render_pass.draw(RECT_VERTEX_RANGE, 0..self.rects.len());
    }
}
