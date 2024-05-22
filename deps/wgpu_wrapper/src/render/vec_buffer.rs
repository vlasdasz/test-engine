use bytemuck::{cast_slice, Pod};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer,
};

use crate::{BufferUsages, WGPUApp};

#[derive(Debug)]
pub(crate) struct VecBuffer<T> {
    len:    u32,
    data:   Vec<T>,
    buffer: Buffer,
}

impl<T: Pod> VecBuffer<T> {
    pub fn push(&mut self, val: T) {
        self.data.push(val)
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn load(&mut self) {
        self.buffer = WGPUApp::device().create_buffer_init(&BufferInitDescriptor {
            label:    Some("Instance Buffer"),
            contents: cast_slice(&self.data),
            usage:    BufferUsages::VERTEX,
        });
        self.len = self.data.len().try_into().unwrap();
        self.data.clear();
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl<T> Default for VecBuffer<T> {
    fn default() -> Self {
        Self {
            len:    0,
            data:   vec![],
            buffer: WGPUApp::device().create_buffer_init(&BufferInitDescriptor {
                label:    Some("empty_buffer"),
                contents: &[],
                usage:    BufferUsages::VERTEX,
            }),
        }
    }
}
