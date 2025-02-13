use wgpu::Buffer;
use window::{Window, utils::ToBytes};

pub(crate) trait BufferHelper {
    fn update<T: ToBytes>(&self, data: T);
}

impl BufferHelper for Buffer {
    fn update<T: ToBytes>(&self, data: T) {
        Window::queue().write_buffer(self, 0, data.to_bytes());
    }
}
