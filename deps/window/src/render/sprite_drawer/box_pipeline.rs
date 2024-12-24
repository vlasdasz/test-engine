use gm::flat::{Point, Size};
use wgpu::{
    Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages, include_wgsl,
};

use crate::{
    Window,
    render::{
        sprite_drawer::shader_data::{
            FULL_SCREEN_VERTEX_RANGE, FULL_SCREEN_VERTICES, SpriteInstance, SpriteView,
        },
        uniform::{UniformBind, make_uniform_layout},
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
};

#[derive(Debug)]
pub struct BoxPipeline {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    view:  UniformBind<SpriteView>,
    boxes: VecBuffer<SpriteInstance>,
}

impl Default for BoxPipeline {
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(include_wgsl!("sprite.wgsl"));

        let sprite_view_layout = make_uniform_layout("sprite_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "sprite_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.pipeline(
            "sprite_driver_pipeline",
            &uniform_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, SpriteInstance::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            vertex_buffer: device.buffer(FULL_SCREEN_VERTICES, BufferUsages::VERTEX),
            view: sprite_view_layout.into(),
            boxes: VecBuffer::default(),
        }
    }
}

impl BoxPipeline {
    pub fn add(&mut self, instance: SpriteInstance) {
        self.boxes.push(instance);
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        scale: f32,
        camera_rotation: f32,
        camera_pos: Point,
        resolution: Size,
    ) {
        if self.boxes.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);

        self.view.update(SpriteView {
            camera_pos,
            resolution,
            camera_rotation,
            scale,
        });

        self.boxes.load();

        render_pass.set_bind_group(0, self.view.bind(), &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.boxes.buffer().slice(..));

        render_pass.draw(FULL_SCREEN_VERTEX_RANGE, 0..self.boxes.len());
    }
}
