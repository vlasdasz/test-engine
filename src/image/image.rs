
// void* data, float width, float height, uint8_t channels

use std::ffi::c_void;
use crate::gm::Size;
use crate::gl_wrapper::TextureLoader;

pub struct Image {
    pub size: Size,
    pub channels: u32,
    gl_handle: u32
}

impl Image {
    pub fn from(data: *const c_void, size: Size, channels: u32) -> Image {
        let gl_handle = TextureLoader::load(data, size, channels);
        Image { size, channels, gl_handle }
    }
}