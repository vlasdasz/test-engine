use std::collections::HashMap;

use bytemuck::cast_slice;
use gm::Color;
use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, ShaderStages,
};

use crate::{BufferUsages, WGPUApp};

static BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();
static LAYOUT: MainLock<Option<BindGroupLayout>> = MainLock::new();

pub trait Uniform<T> {
    fn bind(val: T) -> &'static BindGroup;
    fn layout() -> &'static BindGroupLayout;
}

impl Uniform<Color> for Color {
    fn bind(color: Color) -> &'static BindGroup {
        BINDS.get_mut().entry(color).or_insert_with(|| bind_group_with_color(&color))
    }

    fn layout() -> &'static BindGroupLayout {
        let layout = LAYOUT.get_mut();

        if let Some(layout) = layout {
            return layout;
        }

        *layout = WGPUApp::device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            .into();

        LAYOUT.get_mut().as_ref().unwrap()
    }
}

fn bind_group_with_color(color: &Color) -> BindGroup {
    let device = WGPUApp::device();

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label:    Some("Color Uniform Buffer"),
        contents: cast_slice(&color.as_slice()),
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
