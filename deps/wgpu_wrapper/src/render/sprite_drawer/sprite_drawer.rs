use bytemuck::bytes_of;
use gm::{
    flat::{Point, Size},
    Color,
};
use wgpu::{
    include_wgsl, BindGroup, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{SpriteInstance, SpriteView, VERTEX_RANGE, VERTICES},
        uniform::make_uniform_layout,
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

#[derive(Debug)]
pub struct SpriteDrawer {
    view: SpriteView,

    pipeline: RenderPipeline,

    view_buffer:     Buffer,
    view_bind_group: BindGroup,

    vertex_buffer: Buffer,

    instances: VecBuffer<SpriteInstance>,
}

impl SpriteDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let sprite_view_layout = make_uniform_layout("Sprites View Layout", ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Sprite Pipeline Layout"),
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
            "Sprite Drawer Render Pipeline",
            Some(&uniform_layout),
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
            vertex_buffer: device.buffer(VERTICES, BufferUsages::VERTEX),
            instances: VecBuffer::default(),
        }
    }

    pub fn add_instance(&mut self, size: Size, position: Point, rotation: f32, color: Color) {
        self.instances.push(SpriteInstance {
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
            WGPUApp::queue().write_buffer(&self.view_buffer, 0, bytes_of(&view));
        }

        self.instances.load();

        render_pass.set_bind_group(0, &self.view_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instances.buffer().slice(..));

        render_pass.draw(VERTEX_RANGE, 0..self.instances.len());
    }
}
