use std::{ffi::c_void, hash::Hash, path::Path};

use gl_wrapper::{
    buffers::{Buffers, FrameBuffer},
    image_loader::ImageLoader,
    GLWrapper,
};
use gm::{
    flat::{Rect, Size},
    Color,
};
use image::GenericImageView;
use log::error;
use rtools::{
    data_manager::{DataManager, DataStorage, Handle, LoadFromPath, Managed},
    file::File,
    hash, managed,
};

use crate::shaders::ImageShaders;

#[derive(Debug)]
pub struct Image {
    pub size:      Size,
    pub channels:  u32,
    pub flipped:   bool,
    pub flipped_y: bool,
    gl_handle:     u32,
}

impl Image {
    pub fn empty() -> Self {
        Self {
            size: Default::default(),
            channels: 0,
            flipped: false,
            flipped_y: false,
            gl_handle: u32::MAX,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    pub fn is_invalid(&self) -> bool {
        self.gl_handle == u32::MAX
    }

    fn load_to_gl(data: *const c_void, size: Size, channels: u32) -> Image {
        let gl_handle = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            flipped: false,
            flipped_y: false,
            gl_handle,
        }
    }

    pub fn from(data: *const c_void, size: Size, channels: u32, hash: u64) -> Handle<Image> {
        Image::add_with_hash(hash, Self::load_to_gl(data, size, channels))
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_texture(self.gl_handle)
    }
}

impl Image {
    pub fn draw(
        name: impl ToString + Hash,
        size: impl Into<Size>,
        draw: impl FnOnce(&mut Image),
    ) -> Handle<Image> {
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
        };

        GLWrapper::clear_with_color(Color::GREEN);

        draw(&mut image);

        buffer.unbind();

        Image::add_with_hash(hash, image)
    }
}

managed!(Image);

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
        )
    }
}

pub fn draw_image(image: &Image, rect: &Rect, color: &Color) {

    if image.is_invalid() {
        return;
    }

    let shaders = ImageShaders::get();

    if image.is_monochrome() {
        shaders.monochrome.enable().set_color(color)
    } else {
        shaders.texture.enable()
    }
    .set_flipped(image.flipped)
    .set_flipped_y(image.flipped_y);

    GLWrapper::set_viewport(*rect);

    image.bind();
    Buffers::get().full_image.draw();
}
