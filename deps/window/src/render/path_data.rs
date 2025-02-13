use std::ops::Range;

use gm::{
    Color,
    flat::{Point, Size},
};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindingResource, Buffer, BufferBinding, BufferUsages,
};

use crate::{Window, utils::DeviceHelper};

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

    pub fn bind(&self) -> &BindGroup {
        &self.bind
    }

    pub fn vertex_range(&self) -> Range<u32> {
        self.vertex_range.clone()
    }

    pub fn new(color: Color, size: Size, points: &[Point]) -> Self {
        let device = Window::device();
        let path_layout = Window::path_layout();

        let buffer = device.buffer(points, BufferUsages::VERTEX);

        let bind_group = make_bind_group(path_layout, &color, size);

        Self {
            color,
            buffer,
            bind: bind_group,
            vertex_range: 0..u32::try_from(points.len()).unwrap(),
        }
    }

    pub fn resize(&mut self, size: Size) {
        let path_layout = Window::path_layout();
        self.bind = make_bind_group(path_layout, &self.color, size);
    }
}

fn make_bind_group(bind_group_layout: &BindGroupLayout, color: &Color, size: Size) -> BindGroup {
    let device = Window::device();

    let size_uniform_buffer = device.buffer(&size, BufferUsages::UNIFORM);
    let color_uniform_buffer = device.buffer(color, BufferUsages::UNIFORM);

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label:   Some("path_bind_group"),
        layout:  bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding:  0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &size_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            },
            BindGroupEntry {
                binding:  1,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            },
        ],
    })
}
