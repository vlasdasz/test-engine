#![allow(dead_code)]

use std::{fs::read_to_string, ops::Range};

use bytemuck::Pod;
use gm::{checked_usize_to_u32, flat::Point};
use wgpu::{
    Buffer, PipelineLayoutDescriptor, PrimitiveTopology, RenderPipeline, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};
use window::{
    BufferUsages, DeviceHelper, PolygonMode, UniformBind, VecBuffer, VertexLayout, Window,
    make_uniform_layout,
};

const FULL_SCREEN_VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

pub(super) const FULL_SCREEN_VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(FULL_SCREEN_VERTICES.len());

pub struct Pipeline<const SHADER: &'static str, View, Instance> {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    view:      UniformBind<View>,
    instances: VecBuffer<Instance>,
}

impl<const SHADER: &'static str, View: Default + Pod, Instance: VertexLayout> Default
    for Pipeline<SHADER, View, Instance>
{
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label:  None,
            source: ShaderSource::Wgsl(read_to_string(format!("{SHADER}.wgsl")).expect("Kkoko").into()),
        });

        let sprite_view_layout =
            make_uniform_layout(&format!("{SHADER}_uniform_layout"), ShaderStages::VERTEX_FRAGMENT);

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some(&format!("{SHADER}_pipeline_layout")),
            bind_group_layouts:   &[&sprite_view_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.pipeline(
            &format!("{SHADER}_pipeline"),
            &uniform_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT, Instance::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            vertex_buffer: device.buffer(FULL_SCREEN_VERTICES, BufferUsages::VERTEX),
            view: sprite_view_layout.into(),
            instances: VecBuffer::default(),
        }
    }
}
