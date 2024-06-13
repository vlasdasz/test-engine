use bytemuck::cast_slice;
use gm::{
    checked_usize_to_u32,
    flat::{Point, Points},
    Color,
};
use wgpu::{
    include_wgsl, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages,
};

use crate::{
    render::{
        sprite_drawer::shader_data::SpriteView,
        uniform::{make_uniform_layout, UniformBind},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

#[derive(Debug)]
pub struct PolygonPipeline {
    pipeline: RenderPipeline,

    view:  UniformBind<SpriteView>,
    pos:   UniformBind<Point>,
    color: UniformBind<Color>,

    polygons: Vec<(Buffer, Points, Point, Color)>,
}

impl Default for PolygonPipeline {
    fn default() -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/polygon.wgsl"));

        let view_layout = make_uniform_layout("polygon_view_layout", ShaderStages::VERTEX);
        let pos_layout = make_uniform_layout("polygon_pos_layout", ShaderStages::VERTEX);
        let color_layout = make_uniform_layout("polygon_color_layout", ShaderStages::FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "polygon_pipeline_layout".into(),
            bind_group_layouts:   &[&view_layout, &pos_layout, &color_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.pipeline(
            "polygon_pipeline",
            &uniform_layout,
            &shader,
            PolygonMode::Line,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            view: view_layout.into(),
            pos: pos_layout.into(),
            color: color_layout.into(),
            polygons: vec![],
        }
    }
}

impl PolygonPipeline {
    pub fn add(&mut self, points: Points, pos: Point, color: Color) {
        self.polygons.push((
            WGPUApp::device().buffer_from_bytes(cast_slice(points.as_slice()), BufferUsages::VERTEX),
            points,
            pos,
            color,
        ))
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, view: SpriteView) {
        render_pass.set_pipeline(&self.pipeline);

        self.view.update(view);

        render_pass.set_bind_group(0, &self.view.bind(), &[]);

        for (buffer, points, pos, color) in &self.polygons {
            self.pos.update(*pos);
            self.color.update(*color);

            render_pass.set_bind_group(1, &self.pos.bind(), &[]);
            render_pass.set_bind_group(2, &self.color.bind(), &[]);

            render_pass.set_vertex_buffer(0, buffer.slice(..));

            render_pass.draw(0..checked_usize_to_u32(points.len()), 0..1);
        }
    }
}
