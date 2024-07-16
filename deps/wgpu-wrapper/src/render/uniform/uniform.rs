use std::collections::HashMap;

use bytemuck::Pod;
use gm::Color;
use refs::MainLock;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, ShaderStages,
};

use crate::{utils::DeviceHelper, BufferUsages, WGPUApp};

static Z_BINDS: MainLock<HashMap<u32, BindGroup>> = MainLock::new();
static COLOR_BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();

pub(crate) fn cached_z_bind(z: f32, layout: &BindGroupLayout) -> &'static BindGroup {
    Z_BINDS.get_mut().entry(z.to_bits()).or_insert_with(|| make_bind(&z, layout))
}

pub(crate) fn cached_color_bind(color: Color, layout: &BindGroupLayout) -> &'static BindGroup {
    COLOR_BINDS.get_mut().entry(color).or_insert_with(|| make_bind(&color, layout))
}

pub fn make_bind<T: Pod>(data: &T, layout: &BindGroupLayout) -> BindGroup {
    let device = WGPUApp::device();
    let buffer = device.buffer(data, BufferUsages::UNIFORM);
    device.bind(&buffer, layout)
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
