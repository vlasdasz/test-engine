use wgpu::Buffer;

use crate::{utils::ToBytes, WGPUApp};

pub trait BufferHelper {
    fn update<T: ToBytes>(&self, data: T);
    fn update_bytes(&self, data: &[u8]);
}

impl BufferHelper for Buffer {
    fn update<T: ToBytes>(&self, data: T) {
        WGPUApp::queue().write_buffer(self, 0, data.to_bytes());
    }

    fn update_bytes(&self, data: &[u8]) {
        WGPUApp::queue().write_buffer(self, 0, data);
    }
}
