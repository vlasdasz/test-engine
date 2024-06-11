use std::{collections::HashMap, ops::Range};

use bytemuck::bytes_of;
use gm::{
    checked_usize_to_u32,
    flat::{Point, Size},
    volume::Vertex,
    Color,
};
use refs::Weak;
use wgpu::{
    BindGroup, Buffer, BufferUsages, PolygonMode, PrimitiveTopology, RenderPass, RenderPipeline,
    ShaderStages, TextureFormat,
};

use crate::{
    image::Image,
    render::{
        sprite_drawer::shader_data::{SpriteInstance, SpriteView},
        uniform::make_uniform_layout,
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

const VERTICES: [Vertex; 4] = [
    Vertex {
        pos: Point::new(-1.0, 1.0),
        uv:  Point::new(0.0, 0.0),
    },
    Vertex {
        pos: Point::new(-1.0, -1.0),
        uv:  Point::new(0.0, 1.0),
    },
    Vertex {
        pos: Point::new(1.0, 1.0),
        uv:  Point::new(1.0, 0.0),
    },
    Vertex {
        pos: Point::new(1.0, -1.0),
        uv:  Point::new(1.0, 1.0),
    },
];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct TexturedSpriteDrawer {
    render_pipeline: RenderPipeline,

    view_buffer:     Buffer,
    view_bind_group: BindGroup,

    vertex_buffer: Buffer,

    instances: HashMap<Weak<Image>, VecBuffer<SpriteInstance>>,
}

impl TexturedSpriteDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/sprite_textured.wgsl"));

        let sprite_view_layout = make_uniform_layout("Sprites View Layout", ShaderStages::VERTEX_FRAGMENT);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "Textured Sprite Pipeline Layout".into(),
            bind_group_layouts:   &[&sprite_view_layout, &Image::uniform_layout()],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.pipeline(
            "Textured Sprite Render Pipeline",
            Some(&pipeline_layout),
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Vertex::VERTEX_LAYOUT, SpriteInstance::VERTEX_LAYOUT],
        );

        let vertex_buffer = device.buffer(&VERTICES, BufferUsages::VERTEX);

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

        Self {
            render_pipeline,
            view_buffer,
            view_bind_group,
            vertex_buffer,
            instances: HashMap::default(),
        }
    }

    pub fn add_instance(
        &mut self,
        image: Weak<Image>,
        size: Size,
        position: Point,
        rotation: f32,
        color: Color,
    ) {
        let image = self.instances.entry(image).or_default();

        image.push(SpriteInstance {
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
        render_pass.set_pipeline(&self.render_pipeline);

        let queue = WGPUApp::queue();

        queue.write_buffer(
            &self.view_buffer,
            0,
            bytes_of(&SpriteView {
                camera_pos,
                resolution,
                camera_rotation,
                scale,
            }),
        );

        for (image, instances) in &mut self.instances {
            instances.load();

            render_pass.set_bind_group(0, &self.view_bind_group, &[]);
            render_pass.set_bind_group(1, &image.bind, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

            render_pass.draw(VERTEX_RANGE, 0..instances.len());
        }
    }
}
