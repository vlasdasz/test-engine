use bytemuck::Pod;
use wgpu::Buffer;

use crate::{BufferUsages, WGPUApp, utils::DeviceHelper};

#[derive(Debug)]
pub(crate) struct VecBuffer<T> {
    len:    u32,
    data:   Vec<T>,
    buffer: Buffer,
}

impl<T> VecBuffer<T> {
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl<T: Pod> VecBuffer<T> {
    pub fn load(&mut self) {
        self.buffer = WGPUApp::device().buffer(self.data.as_slice(), BufferUsages::VERTEX);
        self.len = self.data.len().try_into().unwrap();
        self.data.clear();
    }
}

impl<T> Default for VecBuffer<T> {
    fn default() -> Self {
        Self {
            len:    0,
            data:   vec![],
            buffer: WGPUApp::device().buffer(&(), BufferUsages::VERTEX),
        }
    }
}
