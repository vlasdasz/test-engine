use bytemuck::cast_slice;
use gm::{checked_usize_to_u32, flat::Point, Color};
use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveTopology, RenderPass, RenderPipeline, ShaderStages,
};

use crate::{
    render::{
        sprite_drawer::shader_data::SpriteView,
        uniform::{make_bind, make_uniform_layout, UniformBind},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

#[derive(Debug)]
pub struct PolygonPipeline {
    pipeline: RenderPipeline,

    view: UniformBind<SpriteView>,

    pos_layout:   BindGroupLayout,
    color_layout: BindGroupLayout,

    polygons: Vec<(Buffer, usize, BindGroup, BindGroup)>,
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
            pos_layout,
            color_layout,
            polygons: vec![],
        }
    }
}

impl PolygonPipeline {
    pub fn add(&mut self, points: &[Point], pos: Point, color: Color) {
        self.polygons.push((
            WGPUApp::device().buffer_from_bytes(cast_slice(points), BufferUsages::VERTEX),
            points.len(),
            make_bind(&pos, &self.pos_layout),
            make_bind(&color, &self.color_layout),
        ));
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, view: SpriteView) {
        render_pass.set_pipeline(&self.pipeline);

        self.view.update(view);

        render_pass.set_bind_group(0, self.view.bind(), &[]);

        for (buffer, points_len, pos, color) in &self.polygons {
            render_pass.set_bind_group(1, pos, &[]);
            render_pass.set_bind_group(2, color, &[]);

            render_pass.set_vertex_buffer(0, buffer.slice(..));

            render_pass.draw(0..checked_usize_to_u32(*points_len), 0..1);
        }
    }

    pub fn clear(&mut self) {
        self.polygons.clear();
    }
}
