mod font;
mod glyph;

use std::ops::Deref;

pub use font::Font;
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use gl_image::{draw_image, Image};
use gm::{flat::Size, Color};
use rtools::{
    data_manager::{DataManager, Handle},
    IntoF32,
};

pub fn render_text(text: &str, font: &Font, size: impl IntoF32) -> Handle<Image> {
    if text.is_empty() {
        return Default::default();
    }

    if let Some(image) = Image::handle_with_name(text) {
        return image;
    }

    let mut layout: Layout = Layout::new(CoordinateSystem::PositiveYDown);

    layout.append(&[&font.font], &TextStyle::new(text, size.into_f32(), 0));

    let size: Size = {
        let last = layout.glyphs().last().unwrap();
        (last.x + last.width as f32, last.y + last.height as f32).into()
    };

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
