use bytemuck::{Pod, Zeroable};
use educe::Educe;
use gm::{
    flat::{Point, Points, Size},
    Color,
};
use wgpu::{
    include_wgsl, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass, RenderPipeline,
    ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{SpriteBox, SpriteView},
        uniform::{make_uniform_layout, UniformBind},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod, PartialEq, Educe)]
#[educe(Default)]
struct PolygonView {
    pos:        Point,
    camera_pos: Point,
    #[educe(Default = (1000, 1000).into())]
    resolution: Size,
    camera_rot: f32,
    #[educe(Default = 1.0)]
    scale:      f32,
}

#[derive(Debug)]
pub struct PolygonPipeline {
    pipeline: RenderPipeline,
    view:     UniformBind<PolygonView>,
}

impl Default for PolygonPipeline {
    fn default() -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/polygon.wgsl"));

        let sprite_view_layout = make_uniform_layout("polygon_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "polygon_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let view = UniformBind::new(device, &sprite_view_layout);

        let pipeline = device.pipeline(
            "polygon_pipeline",
            &uniform_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, SpriteBox::VERTEX_LAYOUT],
        );

        Self { pipeline, view }
    }
}

impl PolygonPipeline {
    pub fn add(&mut self, _points: &Points, _pos: Point, _rot: f32, _col: Color) {}

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        scale: f32,
        camera_rotation: f32,
        camera_pos: Point,
        resolution: Size,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        let view = SpriteView {
            camera_pos,
            resolution,
            camera_rotation,
            scale,
        };

        // render_pass.set_bind_group(0, &self.view_bind_group, &[]);
        //
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // render_pass.set_vertex_buffer(1, self.boxes.buffer().slice(..));
        //
        // render_pass.draw(FULL_SCREEN_VERTEX_RANGE, 0..self.boxes.len());

        //  self.shapes.load();
    }
}
