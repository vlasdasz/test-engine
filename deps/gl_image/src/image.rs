use std::{ffi::c_void, hash::Hash, mem::size_of, path::Path};

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
use refs::TotalSize;
use rtools::{
    data_manager::{DataManager, DataStorage, Handle, Managed, ResourceLoader},
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
    buffer:        FrameBuffer,
    total_size:    usize,
}

impl Image {
    pub fn empty() -> Self {
        Self {
            size:       Default::default(),
            channels:   0,
            flipped:    false,
            flipped_y:  false,
            buffer:     FrameBuffer {
                buffer_handle:  u32::MAX,
                texture_handle: u32::MAX,
            },
            total_size: size_of::<Self>(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    pub fn is_invalid(&self) -> bool {
        self.buffer.texture_handle == u32::MAX
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn load_to_gl(data: *const c_void, size: Size, channels: u32) -> Image {
        let buffer = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            flipped: false,
            flipped_y: false,
            buffer,
            total_size: size_of::<Image>() + size.square() as usize * channels as usize,
        }
    }

    pub fn from(data: *const c_void, size: Size, channels: u32, hash: u64) -> Handle<Image> {
        Image::add_with_hash(hash, Self::load_to_gl(data, size, channels))
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_texture(self.buffer.texture_handle)
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
            buffer,
            total_size: size_of::<Self>() + 10,
        };

        GLWrapper::clear_with_color(Color::GREEN);

        draw(&mut image);

        GLWrapper::unbind_framebuffer();

        Image::add_with_hash(hash, image)
    }
}

managed!(Image);

impl ResourceLoader for Image {
    fn load_path(path: &Path) -> Self {
        Self::load_data(&File::read(path), path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let image = image::load_from_memory(data).unwrap_or_else(|_| {
            error!("Failed to load image: {}", name.to_string());
            panic!("Failed to load image: {}", name.to_string());
        });

        let dimensions = image.dimensions();
        let data = image.as_bytes();
        let channels = image.color().channel_count();

        Image::load_to_gl(
            data.as_ptr().cast(),
            (dimensions.0, dimensions.1).into(),
            u32::from(channels),
        )
    }
}

pub fn draw_image(image: &Image, rect: &Rect, color: &Color, priority: usize, is_text: bool) {
    if image.is_invalid() {
        return;
    }

    if is_text {
        ImageShaders::text().enable().set_color(color)
    } else if image.is_monochrome() {
        ImageShaders::mono().enable().set_color(color)
    } else {
        ImageShaders::color().enable()
    }
    .set_flipped(image.flipped)
    .set_flipped_y(image.flipped_y)
    .set_priority(priority);

    GLWrapper::set_viewport(*rect);

    image.bind();
    Buffers::get().full_image.draw();
}

impl TotalSize for Image {
    fn total_size(&self) -> usize {
        self.total_size
    }
}
