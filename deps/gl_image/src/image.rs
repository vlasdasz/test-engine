use std::{
    ffi::c_void,
    path::{Path, PathBuf},
};

use gl_wrapper::{buffers::FrameBuffer, image_loader::ImageLoader, GLWrapper};
use gm::flat::Size;
use image::GenericImageView;
use rtools::{
    data_manager::{DataManager, Handle, LoadFromPath},
    file::File,
    misc::hash,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub size:      Size,
    pub channels:  u32,
    pub flipped:   bool,
    pub flipped_y: bool,
    gl_handle:     u32,
    path:          Option<PathBuf>,
}

impl Image {
    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    pub fn is_invalid(&self) -> bool {
        self.gl_handle == u32::MAX
    }

    fn load_to_gl(data: *const c_void, size: Size, channels: u32, path: Option<PathBuf>) -> Image {
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

    pub fn from(
        data: *const c_void,
        size: Size,
        channels: u32,
        hash: u64,
        path: Option<PathBuf>,
    ) -> Handle<Image> {
        Image::add_with_hash(hash, Self::load_to_gl(data, size, channels, path))
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_texture(self.gl_handle)
    }
}

impl Image {
    pub fn draw(name: &str, size: impl Into<Size>, mut draw: impl FnMut(&mut Image)) -> Handle<Image> {
        let hash = hash(name);

        if let Some(image) = Image::handle_with_hash(hash) {
            return image;
        }

        let size = size.into();
        let buffer = FrameBuffer::from(size);

        buffer.bind();

        let mut image = Self {
            size,
            channels: 4,
            flipped: false,
            flipped_y: false,
            gl_handle: buffer.texture_handle,
            path: PathBuf::from("drawn").into(),
        };

        draw(&mut image);

        buffer.unbind();

        Image::add_with_hash(hash, image)
    }
}

impl LoadFromPath for Image {
    fn load(path: &Path) -> Image {
        let image = image::load_from_memory(&File::read(path)).unwrap_or_else(|_| {
            error!("Failed to open image {:?}", path);
            panic!();
        });

        let dimensions = image.dimensions();
        let data = image.as_bytes();
        let channels = image.color().channel_count();

        Image::load_to_gl(
            data.as_ptr() as *const c_void,
            (dimensions.0, dimensions.1).into(),
            channels as u32,
            Some(path.into()),
        )
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
        Self::load(path.as_ref())
    }
}
