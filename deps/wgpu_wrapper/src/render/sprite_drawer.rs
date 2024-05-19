use std::ops::Range;

use bytemuck::{cast_slice, Pod, Zeroable};
use gm::{
    checked_usize_to_u32,
    flat::{Point, Size},
    Color,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, RenderPass, RenderPipeline,
    ShaderStages, TextureFormat,
};

use crate::{
    render::uniform::{make_bind, make_layout},
    utils::make_pipeline,
    WGPUApp,
};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
struct SpriteVertexData {
    size:            Size,
    position:        Point,
    camera_position: Point,
    resolution:      Size,
    rotation:        f32,
    scale:           f32,
    camera_rotation: f32,
    _padding:        u32,
}

#[derive(Debug)]
pub struct SpriteDrawer {
    pipeline:        RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
    fragment_layout: BindGroupLayout,
}

impl SpriteDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/sprite.wgsl"));

        let vertex_layout = make_layout("sprites_vertex_layout", ShaderStages::VERTEX);
        let fragment_layout = make_layout("sprites_fragment_layout", ShaderStages::FRAGMENT);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&vertex_layout, &fragment_layout],
            push_constant_ranges: &[],
        });

        let pipeline = make_pipeline::<Point>(
            "Rect Fill Render Pipeline",
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertex_layout,
            fragment_layout,
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        size: Size,
        position: Point,
        rotation: f32,
        scale: f32,
        camera_rotation: f32,
        camera_position: Point,
        resolution: Size,
        color: Color,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(
            0,
            make_bind(
                &SpriteVertexData {
                    size,
                    position,
                    camera_position,
                    resolution,
                    rotation,
                    scale,
                    camera_rotation,
                    _padding: 0,
                },
                &self.vertex_layout,
            ),
            &[],
        );

        render_pass.set_bind_group(1, make_bind(&color, &self.fragment_layout), &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(VERTEX_RANGE, 0..1);
    }
}
