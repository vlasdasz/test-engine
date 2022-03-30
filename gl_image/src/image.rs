use std::{
    ffi::c_void,
    path::{Path, PathBuf},
};

use gl_wrapper::{image_loader::ImageLoader, GLWrapper};
use gm::Size;
use image::GenericImageView;
use rtools::file::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub size:      Size,
    pub channels:  u32,
    pub flipped:   bool,
    pub flipped_y: bool,
    gl_handle:     u32,
    path:          Option<PathBuf>,
}

impl Image {
    pub fn is_invalid(&self) -> bool {
        self.gl_handle == u32::MAX
    }

    pub fn load(path: impl AsRef<Path>) -> Image {
        let image = image::load_from_memory(&File::read(path.as_ref())).unwrap_or_else(|_| {
            error!("Failed to open image {:?}", path.as_ref());
            panic!();
        });

        let dimensions = image.dimensions();
        let data = image.as_bytes();
        let channels = image.color().channel_count();

        Image::from(
            data.as_ptr() as *const c_void,
            (dimensions.0, dimensions.1).into(),
            channels as u32,
            Some(path.as_ref().into()),
        )
    }

    pub fn from(data: *const c_void, size: Size, channels: u32, path: Option<PathBuf>) -> Image {
        let gl_handle = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            flipped: false,
            flipped_y: false,
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
            size:      Default::default(),
            channels:  0,
            flipped:   false,
            flipped_y: false,
            gl_handle: u32::MAX,
            path:      Default::default(),
        }
    }
}

impl<T: AsRef<Path>> From<T> for Image {
    fn from(path: T) -> Self {
        Self::load(path)
    }
}
