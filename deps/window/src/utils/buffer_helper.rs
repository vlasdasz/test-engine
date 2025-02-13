use wgpu::Buffer;

use crate::{Window, utils::ToBytes};

pub trait BufferHelper {
    fn update<T: ToBytes>(&self, data: T);
    fn update_bytes(&self, data: &[u8]);
}

impl BufferHelper for Buffer {
    fn update<T: ToBytes>(&self, data: T) {
        Window::queue().write_buffer(self, 0, data.to_bytes());
    }

    fn update_bytes(&self, data: &[u8]) {
        Window::queue().write_buffer(self, 0, data);
    }
}
