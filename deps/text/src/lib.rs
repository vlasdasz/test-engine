mod font;
mod glyph;

use std::ops::Deref;

pub use font::*;
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use gl_image::{draw_image, Image};
use gm::{flat::Size, Color};
use rtools::{
    data_manager::{DataManager, Handle},
    hash, IntoF32,
};

pub fn text_layout(text: impl ToString, font: &Font, size: impl IntoF32) -> (Layout, Size) {
    let mut layout: Layout = Layout::new(CoordinateSystem::PositiveYDown);

    let text = text.to_string();

    layout.append(&[&font.font], &TextStyle::new(&text, size.into_f32(), 0));

    let size = layout
        .glyphs()
        .last()
        .map(|last| (last.x + last.width as f32, last.y + last.height as f32).into())
        .unwrap_or_default();

    (layout, size)
}

pub fn text_size(text: impl ToString, font: &Font, size: impl IntoF32) -> Size {
    text_layout(text, font, size).1
}

pub fn render_text(text: &str, font: &Font, size: impl IntoF32) -> Handle<Image> {
    if let Some(image) = Image::handle_with_name(text) {
        return image;
    }

    if text.is_empty() {
        return Image::add_with_hash(hash(text), Image::empty());
    }

    let (layout, size) = text_layout(text, font, size);

    Image::draw(text, size, |image| {
        for glyph in layout.glyphs() {
            let image = font.glyph_for_char(glyph.parent).image;
            draw_image(
                image.deref(),
                &(
                    glyph.x,
                    size.height - glyph.y - glyph.height as f32 + 10.0,
                    glyph.width,
                    glyph.height,
                )
                    .into(),
                &Color::WHITE, // See ui_monochrome shader
            );
        }

        image.flipped_y = true;
        image.channels = 1;
    })
}
