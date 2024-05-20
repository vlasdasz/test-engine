#![allow(dead_code)]

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer,
};

use crate::{BufferUsages, WGPUApp};

pub(crate) struct VecBuffer<T> {
    data:   Vec<T>,
    buffer: Buffer,
}

impl<T> VecBuffer<T> {
    pub fn new(usage: BufferUsages) -> Self {
        Self {
            data:   vec![],
            buffer: WGPUApp::device().create_buffer_init(&BufferInitDescriptor {
                label: Some("empty_buffer"),
                contents: &[],
                usage,
            }),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val)
    }
}
