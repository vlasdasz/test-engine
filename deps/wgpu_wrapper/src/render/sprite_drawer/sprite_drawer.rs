use bytemuck::{bytes_of, cast_slice};
use gm::{
    flat::{Point, Size},
    Color,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveTopology, RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::{
        sprite_drawer::shader_data::{SpriteInstance, SpriteView, VERTICES},
        uniform::make_uniform_layout,
        vertex_layout::VertexLayout,
    },
    utils::make_pipeline,
    WGPUApp,
};

#[derive(Debug)]
pub struct SpriteDrawer {
    pipeline: RenderPipeline,

    _view_buffer:     Buffer,
    _view_bind_group: BindGroup,

    vertex_buffer:   Buffer,
    instance_buffer: Buffer,

    sprite_view_layout: BindGroupLayout,

    instances: Vec<SpriteInstance>,
}

impl SpriteDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let sprite_view_layout = make_uniform_layout("Sprites View Layout", ShaderStages::VERTEX);

        let _uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Sprite Pipeline Layout"),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let _view_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Camera Buffer"),
            contents: bytes_of(&SpriteView {
                camera_pos:      Default::default(),
                resolution:      (1000, 1000).into(),
                camera_rotation: 0.0,
                scale:           0.0,
            }),
            usage:    BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let _view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &sprite_view_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: _view_buffer.as_entire_binding(),
            }],
            label:   Some("Sprite View Bind Group"),
        });

        let pipeline = make_pipeline(
            "Sprite Drawer Render Pipeline",
            // Some(&uniform_layout),
            None,
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
            _view_buffer,
            _view_bind_group,
            vertex_buffer,
            instance_buffer,
            sprite_view_layout,
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

        let _queue = WGPUApp::queue();

        // queue.write_buffer(
        //     &self.view_buffer,
        //     0,
        //     bytes_of(&SpriteView {
        //         camera_pos,
        //         resolution,
        //         camera_rotation,
        //         scale: 0.0,
        //     }),
        // );

        let device = WGPUApp::device();

        self.instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Instance Buffer"),
            contents: cast_slice(&self.instances),
            usage:    BufferUsages::VERTEX,
        });

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.draw(0..4, 0..self.instances.len() as _);

        self.instances.clear();
    }

    pub fn add_instance(&mut self, size: Size, position: Point, rotation: f32, _color: Color) {
        self.instances.push(SpriteInstance {
            size,
            position,
            rotation,
            paddind: 0,
        });
    }
}
