use std::{any::type_name, collections::HashMap};

use bytemuck::cast_slice;
use gm::Color;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, ShaderStages,
};

use crate::{BufferUsages, WGPUApp};

static LAYOUTS: MainLock<HashMap<&'static str, BindGroupLayout>> = MainLock::const_new();
static BINDS_BUFFER: MainLock<Vec<BindGroup>> = MainLock::const_new();

pub trait Uniform<T>: Sized {
    fn make_bind(self) -> BindGroup;
    fn make_layout() -> BindGroupLayout;

    fn bind(self) -> &'static BindGroup {
        to_ref(self.make_bind())
    }

    fn layout() -> &'static BindGroupLayout {
        LAYOUTS.get_mut().entry(type_name::<T>()).or_insert_with(Self::make_layout)
    }
}

impl Uniform<f32> for f32 {
    fn make_bind(self) -> BindGroup {
        let device = WGPUApp::device();

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("f32_uniform_buffer"),
            contents: cast_slice(&[self]),
            usage:    BufferUsages::UNIFORM,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("f32_bind_group"),
            layout:  Self::layout(),
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

    fn make_layout() -> BindGroupLayout {
        WGPUApp::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   "f32_uniform_bind_group".into(),
            entries: &[BindGroupLayoutEntry {
                binding:    0,
                visibility: ShaderStages::VERTEX,
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

impl Uniform<Color> for Color {
    fn make_bind(self) -> BindGroup {
        let device = WGPUApp::device();

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Color Uniform Buffer"),
            contents: cast_slice(&self.as_slice()),
            usage:    BufferUsages::UNIFORM,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("color_bind_group"),
            layout:  Color::layout(),
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

    fn make_layout() -> BindGroupLayout {
        WGPUApp::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("color_bind_group_layout"),
            entries: &[BindGroupLayoutEntry {
                binding:    0,
                visibility: ShaderStages::FRAGMENT,
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

fn to_ref(bind: BindGroup) -> &'static BindGroup {
    let buf = BINDS_BUFFER.get_mut();
    buf.push(bind);
    buf.last().unwrap()
}

pub(crate) fn clear_uniform_buffer() {
    BINDS_BUFFER.get_mut().clear();
}
