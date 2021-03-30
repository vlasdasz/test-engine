
// void* data, float width, float height, uint8_t channels

use std::ffi::c_void;
use crate::gm::Size;
use crate::gl_wrapper::{TextureLoader, GLWrapper};

#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub size: Size,
    pub channels: u32,
    gl_handle: u32
}

impl Image {

    pub fn new() -> Image {
        Image { size: Size::new(), channels: 0, gl_handle: u32::MAX }
    }

    pub fn from(data: *const c_void, size: Size, channels: u32) -> Image {
        let gl_handle = TextureLoader::load(data, size, channels);
        Image { size, channels, gl_handle }
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_image(self.gl_handle)
    }
}