use std::ops::Range;

use gm::{
    color::Color,
    flat::{Point, Size},
};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
    BufferBinding, BufferBindingType, BufferUsages, ShaderStages,
};
use window::Window;

use crate::device_helper::DeviceHelper;

#[derive(Debug)]
pub struct PathData {
    pub color:    Color,
    buffer:       Buffer,
    bind:         BindGroup,
    vertex_range: Range<u32>,
}

impl PathData {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn uniform_bind(&self) -> &BindGroup {
        &self.bind
    }

    pub fn vertex_range(&self) -> Range<u32> {
        self.vertex_range.clone()
    }

    pub fn new(color: Color, resolution: Size, position: Point, points: &[Point]) -> Self {
        let device = Window::device();

        let buffer = device.buffer(points, BufferUsages::VERTEX);

        let bind_group = make_bind_group(&color, position, resolution);

        Self {
            color,
            buffer,
            bind: bind_group,
            vertex_range: 0..u32::try_from(points.len()).unwrap(),
        }
    }

    pub fn resize(&mut self, position: Point) {
        self.bind = make_bind_group(&self.color, position, Window::render_size());
    }
}

fn make_bind_group(color: &Color, position: Point, resolution: Size) -> BindGroup {
    let device = Window::device();

    let color_uniform_buffer = device.buffer(color, BufferUsages::UNIFORM);
    let position_uniform_buffer = device.buffer(&position, BufferUsages::UNIFORM);
    let resolution_uniform_buffer = device.buffer(&resolution, BufferUsages::UNIFORM);

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label:   Some("path_bind_group"),
        layout:  &PathData::uniform_layout(),
        entries: &[
            BindGroupEntry {
                binding:  0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &position_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            },
            BindGroupEntry {
                binding:  1,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &resolution_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            },
            BindGroupEntry {
                binding:  2,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            },
        ],
    })
}

impl PathData {
    pub fn uniform_layout() -> BindGroupLayout {
        Window::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("path_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding:    0,
                    visibility: ShaderStages::VERTEX,
                    ty:         BindingType::Buffer {
                        ty:                 BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:   None,
                    },
                    count:      None,
                },
                BindGroupLayoutEntry {
                    binding:    1,
                    visibility: ShaderStages::VERTEX,
                    ty:         BindingType::Buffer {
                        ty:                 BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:   None,
                    },
                    count:      None,
                },
                BindGroupLayoutEntry {
                    binding:    2,
                    visibility: ShaderStages::FRAGMENT,
                    ty:         BindingType::Buffer {
                        ty:                 BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:   None,
                    },
                    count:      None,
                },
            ],
        })
    }
}
