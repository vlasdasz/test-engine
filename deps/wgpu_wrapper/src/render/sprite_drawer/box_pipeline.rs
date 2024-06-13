use gm::{
    flat::{Point, Size},
    Color,
};
use wgpu::{
    include_wgsl, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{SpriteBox, SpriteView, FULL_SCREEN_VERTEX_RANGE, FULL_SCREEN_VERTICES},
        uniform::{make_uniform_layout, UniformBind},
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::{BufferHelper, DeviceHelper},
    WGPUApp,
};

#[derive(Debug)]
pub struct BoxPipeline {
    pipeline: RenderPipeline,

    view: UniformBind<SpriteView>,

    vertex_buffer: Buffer,

    boxes: VecBuffer<SpriteBox>,
}

impl Default for BoxPipeline {
    fn default() -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let sprite_view_layout = make_uniform_layout("sprite_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "sprite_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let view = UniformBind::new(device, &sprite_view_layout);

        let pipeline = device.pipeline(
            "sprite_driver_pipeline",
            &uniform_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, SpriteBox::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            view,
            vertex_buffer: device.buffer(FULL_SCREEN_VERTICES, BufferUsages::VERTEX),
            boxes: VecBuffer::default(),
        }
    }
}

impl BoxPipeline {
    pub fn add(&mut self, size: Size, position: Point, rotation: f32, color: Color) {
        self.boxes.push(SpriteBox {
            size,
            position,
            color,
            rotation,
            paddind: 0,
        });
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        scale: f32,
        camera_rotation: f32,
        camera_pos: Point,
        resolution: Size,
    ) {
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
