use std::ops::Range;

use bytemuck::cast_slice;
use gm::{
    flat::{Points, Size},
    Color,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindingResource, Buffer, BufferBinding, BufferUsages, Device,
    PolygonMode,
};

use crate::WGPUApp;

#[derive(Debug)]
pub struct PathData {
    pub mode:     PolygonMode,
    pub color:    Color,
    buffer:       Buffer,
    bind_group:   BindGroup,
    vertex_range: Range<u32>,
}

impl PathData {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn vertex_range(&self) -> Range<u32> {
        self.vertex_range.clone()
    }

    pub fn new(color: Color, size: Size, points: Points, mode: PolygonMode) -> Self {
        let device = &WGPUApp::current().state.drawer.device;
        let path_layout = WGPUApp::path_layout();

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    "PathData Buffer".into(),
            contents: cast_slice(&points),
            usage:    BufferUsages::VERTEX,
        });

        let bind_group = make_bind_group(path_layout, device, &color, size);

        Self {
            mode,
            color,
            buffer,
            bind_group,
            vertex_range: 0..u32::try_from(points.len()).unwrap(),
        }
    }

    pub fn resize(&mut self, size: Size) {
        let device = &WGPUApp::current().state.drawer.device;
        let path_layout = WGPUApp::path_layout();
        self.bind_group = make_bind_group(path_layout, device, &self.color, size);
    }
}

fn make_bind_group(
    bind_group_layout: &BindGroupLayout,
    device: &Device,
    color: &Color,
    size: Size,
) -> BindGroup {
    let size_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("Path Color Uniform Buffer"),
        contents: cast_slice(&size.as_slice()),
        usage:    BufferUsages::UNIFORM,
    });

    let color_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("Path Color Uniform Buffer"),
        contents: cast_slice(&color.as_slice()),
        usage:    BufferUsages::UNIFORM,
    });

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
