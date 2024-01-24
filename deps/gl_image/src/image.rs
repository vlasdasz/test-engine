use std::{mem::size_of, path::Path};

use gl_wrapper::{
    buffers::{Buffers, FrameBuffer},
    image_loader::ImageLoader,
    path_data::{initialize_path_data, DrawMode},
    shaders::BasicShaders,
    GLWrapper,
};
use gm::{
    flat::{Points, ProcessPoints, Rect, Size},
    Color,
};
use image::GenericImageView;
use log::{error, warn};
use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};
use refs::{TotalSize, Weak};
use rtools::file::File;

use crate::shaders::ImageShaders;

#[derive(Debug)]
pub struct Image {
    pub size:      Size,
    pub channels:  u32,
    pub flipped:   bool,
    pub flipped_y: bool,
    buffer:        FrameBuffer,
    total_size:    usize,
    #[allow(dead_code)]
    name:          String,
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
            name:       "empty".to_string(),
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.buffer.texture_handle == u32::MAX
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn load_to_gl(data: &[u8], size: Size, channels: u32, name: String) -> Image {
        let buffer = ImageLoader::load(data, size, channels);
        Image {
            size,
            channels,
            flipped: false,
            flipped_y: false,
            buffer,
            total_size: size_of::<Image>() + size.area() as usize * channels as usize,
            name,
        }
    }

    pub fn from(data: &[u8], size: Size, channels: u32, name: String) -> Weak<Image> {
        if let Some(existing) = Image::weak_with_name(&name) {
            warn!("Image with name: {name} already exists.");
            return existing;
        }
        Image::add_with_name(name.clone(), Self::load_to_gl(data, size, channels, name))
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_texture(self.buffer.texture_handle)
    }
}

impl Image {
    pub fn render(name: impl ToString, size: impl Into<Size>, draw: impl FnOnce(&mut Image)) -> Weak<Image> {
        let name = name.to_string();

        if let Some(image) = Image::weak_with_name(&name) {
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
            name: name.clone(),
        };

        GLWrapper::clear_with_color(Color::CLEAR);

        draw(&mut image);

        GLWrapper::unbind_framebuffer();

        Image::add_with_name(name, image)
    }

    pub fn render_path(name: impl ToString, color: Color, path: Points, draw_mode: DrawMode) -> Weak<Image> {
        let size = path.max_size();

        let path = initialize_path_data(path, &color, draw_mode);

        dbg!(&size);

        Self::render(name, size, |image| {
            GLWrapper::set_viewport(size);
            GLWrapper::clear_with_color(Color::WHITE.with_alpha(0.0));
            BasicShaders::path().enable().set_color(&color).set_size(size);
            path.buffer.draw_with_mode(path.draw_mode.to_gl());

            dbg!(GLWrapper::read_pixel((5, 5).into()));
            dbg!(GLWrapper::read_pixel((1, 1).into()));
            dbg!(GLWrapper::read_pixel((0, 0).into()));
            image.flipped_y = true;
        })
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
            data,
            (dimensions.0, dimensions.1).into(),
            u32::from(channels),
            name.to_string(),
        )
    }
}

pub fn draw_image(image: &Image, rect: &Rect, color: &Color, priority: usize, is_text: bool) {
    if image.is_invalid() {
        return;
    }

    if is_text {
        ImageShaders::text().enable()
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
