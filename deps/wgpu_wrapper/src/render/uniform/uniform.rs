use std::collections::HashMap;

use bytemuck::{cast_slice, Pod};
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, ShaderStages,
};

use crate::{BufferUsages, WGPUApp};

static LAYOUTS: MainLock<HashMap<ShaderStages, BindGroupLayout>> = MainLock::const_new();
static BINDS: MainLock<Vec<BindGroup>> = MainLock::const_new();

pub(super) fn bind_group_to_ref(bind: BindGroup) -> &'static BindGroup {
    let buf = BINDS.get_mut();
    buf.push(bind);
    buf.last().unwrap()
}

pub(crate) fn clear_uniform_buffers() {
    BINDS.get_mut().clear();
}

pub(crate) fn make_bind<T: Pod>(data: T, binding: u32, shader: ShaderStages) -> &'static BindGroup {
    let device = WGPUApp::device();

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("uniform_buffer"),
        contents: cast_slice(&[data]),
        usage:    BufferUsages::UNIFORM,
    });

    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label:   Some("uniform bind group"),
        layout:  make_layout(shader),
        entries: &[BindGroupEntry {
            binding,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size:   None,
            }),
        }],
    });

    bind_group_to_ref(bind)
}

pub(crate) fn make_layout(shader: ShaderStages) -> &'static BindGroupLayout {
    LAYOUTS.get_mut().entry(shader).or_insert_with(|| {
        WGPUApp::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("uniform_bind_group_layout"),
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
    })
}
