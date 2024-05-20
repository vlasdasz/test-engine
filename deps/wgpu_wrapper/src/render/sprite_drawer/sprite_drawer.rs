use bytemuck::{bytes_of, cast_slice};
use gm::{
    flat::{Point, Size},
    Color,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{SpriteInstance, SpriteView, VERTEX_RANGE, VERTICES},
        uniform::make_uniform_layout,
        vertex_layout::VertexLayout,
    },
    utils::make_pipeline,
    WGPUApp,
};

#[derive(Debug)]
pub struct SpriteDrawer {
    pipeline: RenderPipeline,

    view_buffer:     Buffer,
    view_bind_group: BindGroup,

    vertex_buffer:   Buffer,
    instance_buffer: Buffer,

    instances: Vec<SpriteInstance>,
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

        let view_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Camera Buffer"),
            contents: bytes_of(&SpriteView {
                camera_pos:      Default::default(),
                resolution:      (1000, 1000).into(),
                camera_rotation: 0.0,
                scale:           1.0,
            }),
            usage:    BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &sprite_view_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: view_buffer.as_entire_binding(),
            }],
            label:   Some("Sprite View Bind Group"),
        });

        let pipeline = make_pipeline(
            "Sprite Drawer Render Pipeline",
            Some(&uniform_layout),
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, SpriteInstance::VERTEX_LAYOUT],
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Instance Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            view_buffer,
            view_bind_group,
            vertex_buffer,
            instance_buffer,
            instances: vec![],
        }
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        camera_rotation: f32,
        camera_pos: Point,
        resolution: Size,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        let queue = WGPUApp::queue();

        queue.write_buffer(
            &self.view_buffer,
            0,
            bytes_of(&SpriteView {
                camera_pos,
                resolution,
                camera_rotation,
                scale: 1.0,
            }),
        );

        let device = WGPUApp::device();

        self.instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Instance Buffer"),
            contents: cast_slice(&self.instances),
            usage:    BufferUsages::VERTEX,
        });

        render_pass.set_bind_group(0, &self.view_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.draw(VERTEX_RANGE, 0..self.instances.len().try_into().unwrap());

        self.instances.clear();
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
}
