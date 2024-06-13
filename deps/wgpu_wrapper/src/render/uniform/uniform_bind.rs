use bytemuck::Pod;
use gm::flat::Point;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};

use crate::{
    utils::{BufferHelper, DeviceHelper},
    BufferUsages,
};

#[derive(Debug)]
pub struct UniformBind<T> {
    data:   T,
    buffer: Buffer,
    bind:   BindGroup,
}

impl<T> UniformBind<T> {
    pub fn bind(&self) -> &BindGroup {
        &self.bind
    }
}

impl<T: Default + Pod> UniformBind<T> {
    pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
        let data = T::default();
        let buffer = device.buffer(&data, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let bind = device.bind(&buffer, &layout);
        Self { data, buffer, bind }
    }
}

impl<T: Pod + PartialEq> UniformBind<T> {
    pub fn update(&mut self, data: T) {
        if self.data == data {
            return;
        }
        self.buffer.update(data);
        self.data = data;
    }
}
