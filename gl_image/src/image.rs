use std::{
    default::default,
    ffi::c_void,
    path::{Path, PathBuf},
};

use gl_wrapper::{image_loader::ImageLoader, GLWrapper};
use gm::Size;
use image::GenericImageView;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Image {
    pub size:     Size,
    pub channels: u32,
    gl_handle:    u32,
    path:         Option<PathBuf>,
}

impl Image {
    pub fn is_invalid(&self) -> bool {
        self.gl_handle == u32::MAX
    }

    pub fn load(path: &Path) -> Image {
        let image = image::open(path).unwrap_or_else(|_| panic!("Failed to open image {:?}", path));

        let dimensions = image.dimensions();
        let data = image.as_bytes();
        let channels = image.color().channel_count();

        Image::from(
            data.as_ptr() as *const c_void,
            (dimensions.0, dimensions.1).into(),
            channels as u32,
            Some(path.into()),
        )
    }

    pub fn from(data: *const c_void, size: Size, channels: u32, path: Option<PathBuf>) -> Image {
        let gl_handle = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            gl_handle,
            path,
        }
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_image(self.gl_handle)
    }
}

impl Default for Image {
    fn default() -> Image {
        Image {
            size:      default(),
            channels:  0,
            gl_handle: u32::MAX,
            path:      default(),
        }
    }
}
