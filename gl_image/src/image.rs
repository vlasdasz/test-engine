use std::{ffi::c_void, path::Path};

use gl_wrapper::{image_loader::ImageLoader, GLWrapper};
use gm::Size;
use image::GenericImageView;
use tools::{new, New};

#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub size:     Size,
    pub channels: u32,
    gl_handle:    u32,
}

impl Image {
    pub fn is_invalid(&self) -> bool { self.gl_handle == u32::MAX }

    pub fn load(path: &Path) -> Image { Image::load_with_image(path) }

    pub fn load_with_image(path: &Path) -> Image {
        let image = image::open(path).unwrap_or_else(|_| panic!("Failed to open image {:?}", path));

        let dimensions = image.dimensions();

        let data = image.as_bytes();

        let channels = image.color().channel_count();

        let size = Size {
            width:  dimensions.0 as f32,
            height: dimensions.1 as f32,
        };

        Image::from(data.as_ptr() as *const c_void, size, channels as u32)
    }

    pub fn from(data: *const c_void, size: Size, channels: u32) -> Image {
        let gl_handle = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            gl_handle,
        }
    }

    pub fn is_monochrome(&self) -> bool { self.channels == 1 }

    pub fn bind(&self) { GLWrapper::bind_image(self.gl_handle) }
}

impl New for Image {
    fn new() -> Image {
        Image {
            size:      new(),
            channels:  0,
            gl_handle: u32::MAX,
        }
    }
}
