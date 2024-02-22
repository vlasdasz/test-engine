use std::collections::HashMap;

use bytemuck::cast_slice;
use gm::Color;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, BufferUsages, Device, ShaderStages,
};

static COLOR_BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();
static Z_BINDS: MainLock<HashMap<u32, BindGroup>> = MainLock::new();

pub struct Uniform {}

impl Uniform {
    pub fn color(device: &Device, layout: &BindGroupLayout, color: &Color) -> &'static BindGroup {
        COLOR_BINDS
            .get_mut()
            .entry(*color)
            .or_insert_with(|| bind_group_with_color(layout, device, color))
    }

    pub fn z(device: &Device, layout: &BindGroupLayout, z: f32) -> &'static BindGroup {
        Z_BINDS
            .get_mut()
            .entry(z.to_bits())
            .or_insert_with(|| z_bind_group(layout, device, z))
    }
}

impl Uniform {
    pub fn color_layout(device: &Device) -> BindGroupLayout {
        Self::layout(device, "color_bind_group_layout", ShaderStages::FRAGMENT)
    }

    pub fn z_layout(device: &Device) -> BindGroupLayout {
        Self::layout(device, "z_position_bind_group_layout", ShaderStages::VERTEX)
    }

    fn layout(device: &Device, label: &'static str, shader: ShaderStages) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: label.into(),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: shader,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}

fn bind_group_with_color(layout: &BindGroupLayout, device: &Device, color: &Color) -> BindGroup {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Color Uniform Buffer"),
        contents: cast_slice(&color.as_slice()),
        usage: BufferUsages::UNIFORM,
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rect_color_bind_group"),
        layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    })
}

fn z_bind_group(layout: &BindGroupLayout, device: &Device, z: f32) -> BindGroup {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Color Uniform Buffer"),
        contents: cast_slice(&[z]),
        usage: BufferUsages::UNIFORM,
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rect_z_position_bind_group"),
        layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    })
}
