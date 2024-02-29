use std::collections::HashMap;

use bytemuck::cast_slice;
use gm::Color;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindingResource, BufferBinding,
};

use crate::{BufferUsages, WGPUApp};

static _COLOR_BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();

pub trait _Uniform<T> {
    fn bind(val: T) -> &'static BindGroup;
}

pub struct _ColorUniform {}

impl _Uniform<Color> for _ColorUniform {
    fn bind(_color: Color) -> &'static BindGroup {
        todo!()
    }
}

fn _bind_group_with_color(layout: &BindGroupLayout, color: &Color) -> BindGroup {
    let device = WGPUApp::device();

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("Color Uniform Buffer"),
        contents: cast_slice(&color.as_slice()),
        usage:    BufferUsages::UNIFORM,
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rect_color_bind_group"),
        layout,
        entries: &[BindGroupEntry {
            binding:  0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size:   None,
            }),
        }],
    })
}
