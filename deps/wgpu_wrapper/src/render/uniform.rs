use std::collections::HashMap;

use bytemuck::cast_slice;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, BufferUsages, ShaderStages,
};

use crate::WGPUApp;

static Z_BINDS: MainLock<HashMap<u32, BindGroup>> = MainLock::const_new();

pub struct OldUniform {}

impl OldUniform {
    pub fn z(layout: &BindGroupLayout, z: f32) -> &'static BindGroup {
        Z_BINDS.get_mut().entry(z.to_bits()).or_insert_with(|| z_bind_group(layout, z))
    }
}

impl OldUniform {
    pub fn z_layout() -> BindGroupLayout {
        Self::layout("z_position_bind_group_layout", ShaderStages::VERTEX)
    }

    fn layout(label: &'static str, shader: ShaderStages) -> BindGroupLayout {
        WGPUApp::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   label.into(),
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
}

fn z_bind_group(layout: &BindGroupLayout, z: f32) -> BindGroup {
    let device = WGPUApp::device();

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("Color Uniform Buffer"),
        contents: cast_slice(&[z]),
        usage:    BufferUsages::UNIFORM,
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rect_z_position_bind_group"),
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
