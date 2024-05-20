use std::collections::HashMap;

use bytemuck::{bytes_of, Pod};
use gm::Color;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, BufferBinding, BufferBindingType, ShaderStages,
};

use crate::{BufferUsages, WGPUApp};

static Z_BINDS: MainLock<HashMap<u32, BindGroup>> = MainLock::new();
static COLOR_BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();

pub(crate) fn cached_z_bind(z: f32, layout: &BindGroupLayout) -> &'static BindGroup {
    Z_BINDS
        .get_mut()
        .entry(z.to_bits())
        .or_insert_with(|| make_bind_internal(&z, layout))
}

pub(crate) fn cached_color_bind(color: Color, layout: &BindGroupLayout) -> &'static BindGroup {
    COLOR_BINDS
        .get_mut()
        .entry(color)
        .or_insert_with(|| make_bind_internal(&color, layout))
}

fn make_bind_internal<T: Pod>(data: &T, layout: &BindGroupLayout) -> BindGroup {
    let device = WGPUApp::device();

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("uniform_buffer"),
        contents: bytes_of(data),
        usage:    BufferUsages::UNIFORM,
    });

    let entry = BindGroupEntry {
        binding:  0,
        resource: BindingResource::Buffer(BufferBinding {
            buffer: &buffer,
            offset: 0,
            size:   None,
        }),
    };

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniform bind group"),
        layout,
        entries: &[entry],
    })
}

pub(crate) fn make_uniform_layout(name: &'static str, shader: ShaderStages) -> BindGroupLayout {
    WGPUApp::device().create_bind_group_layout(&BindGroupLayoutDescriptor {
        label:   name.into(),
        entries: &[BindGroupLayoutEntry {
            binding:    0,
            visibility: shader,
            ty:         BindingType::Buffer {
                ty:                 BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size:   None,
            },
            count:      None,
        }],
    })
}
