use gm::{
    flat::{Point, Points, Size},
    Color,
};
use wgpu::{
    include_wgsl, BindGroup, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{
            SpriteInstance, SpriteView, FULL_SCREEN_VERTEX_RANGE, FULL_SCREEN_VERTICES,
        },
        uniform::make_uniform_layout,
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::{BufferHelper, DeviceHelper},
    WGPUApp,
};

#[derive(Debug)]
pub struct SpriteDrawer {
    view: SpriteView,

    pipeline: RenderPipeline,

    view_buffer:     Buffer,
    view_bind_group: BindGroup,

    vertex_buffer: Buffer,

    boxes: VecBuffer<SpriteInstance>,

    shapes: Vec<Points>,
}

impl SpriteDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let sprite_view_layout = make_uniform_layout("sprites_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "sprite_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let view_buffer = device.buffer(
            &SpriteView {
                camera_pos:      Point::default(),
                resolution:      (1000, 1000).into(),
                camera_rotation: 0.0,
                scale:           1.0,
            },
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let view_bind_group = device.bind(&view_buffer, &sprite_view_layout);

        let pipeline = device.pipeline(
            "sprite_driver_pipeline",
            &uniform_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, SpriteInstance::VERTEX_LAYOUT],
        );

        Self {
            view: SpriteView::default(),
            pipeline,
            view_buffer,
            view_bind_group,
            vertex_buffer: device.buffer(FULL_SCREEN_VERTICES, BufferUsages::VERTEX),
            boxes: VecBuffer::default(),
            shapes: Vec::default(),
        }
    }

    pub fn add_box(&mut self, size: Size, position: Point, rotation: f32, color: Color) {
        self.boxes.push(SpriteInstance {
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

        let view = SpriteView {
            camera_pos,
            resolution,
            camera_rotation,
            scale,
        };

        if view != self.view {
            self.view = view;
            self.view_buffer.update(view);
        }

        self.boxes.load();

        render_pass.set_bind_group(0, &self.view_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.boxes.buffer().slice(..));

        render_pass.draw(FULL_SCREEN_VERTEX_RANGE, 0..self.boxes.len());

        //  self.shapes.load();
    }
}
