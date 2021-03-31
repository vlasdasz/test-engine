
// void* data, float width, float height, uint8_t channels

use std::ffi::c_void;
use crate::gm::Size;
use crate::gl_wrapper::{TextureLoader, GLWrapper};
use std::path::PathBuf;
use soil2::{SOIL_load_image, SOIL_free_image_data};
use std::os::raw::c_int;

use crate::check_gl_error;

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

    pub fn load(path: &PathBuf) -> Image {

        log!(path);

        unsafe {

            let mut width: c_int = -1;
            let mut height: c_int = -1;
            let mut channels: c_int = -1;

            let data = SOIL_load_image(
                path.to_str().unwrap().as_ptr() as *const i8,
                &mut width,
                &mut height,
                &mut channels,
                4 //SOIL_LOAD_RGBA
            );

            check_gl_error!();

            let image = Image::from(
                data as *const c_void,
                Size { width: width as f32, height: height as f32 },
                channels as u32
            );

            SOIL_free_image_data(data);

            check_gl_error!();

            image
        }
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