use refs::MainLock;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBinding, BufferBindingType, ShaderStages,
};

use crate::{BufferUsages, WGPUApp};

static BINDS: MainLock<Vec<BindGroup>> = MainLock::const_new();

pub(super) fn bind_group_to_ref(bind: BindGroup) -> &'static BindGroup {
    let buf = BINDS.get_mut();
    buf.push(bind);
    buf.last().unwrap()
}

pub(crate) fn clear_uniform_buffers() {
    BINDS.get_mut().clear();
}

pub(crate) fn make_bind<const SIZE: usize>(
    data: [&[u8]; SIZE],
    layout: &BindGroupLayout,
) -> &'static BindGroup {
    let device = WGPUApp::device();

    let buffers: Vec<_> = data
        .into_iter()
        .map(|contents| {
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("uniform_buffer"),
                contents,
                usage: BufferUsages::UNIFORM,
            })
        })
        .collect();

    let entries: Vec<_> = buffers
        .iter()
        .enumerate()
        .map(|(binding, buffer)| BindGroupEntry {
            binding:  binding.try_into().unwrap(),
            resource: BindingResource::Buffer(BufferBinding {
                buffer,
                offset: 0,
                size: None,
            }),
        })
        .collect();

    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniform bind group"),
        layout,
        entries: &entries,
    });

    bind_group_to_ref(bind)
}

pub(crate) fn make_layout(name: &'static str, shader: ShaderStages, binds_count: u32) -> BindGroupLayout {
    let entries: Vec<_> = (0..binds_count)
        .map(|binding| BindGroupLayoutEntry {
            binding,
            visibility: shader,
            ty: BindingType::Buffer {
                ty:                 BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size:   None,
            },
            count: None,
        })
        .collect();

    WGPUApp::device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label:   name.into(),
        entries: &entries,
    })
}
