mod font;
mod glyph;

use std::ops::Deref;

pub use font::*;
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use gl_image::{draw_image, Image};
use gm::{flat::Size, Color};
use rtools::{
    data_manager::{DataManager, Handle},
    IntoF32,
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
    let id = format!("{text}:size-{size}");

    if let Some(image) = Image::handle_with_name(&id) {
        return image;
    }

    if text.is_empty() {
        return Image::add_with_name(id, Image::empty());
    }

    let (layout, size) = text_layout(text, font, size);

    if !Font::render_enabled() {
        let mut image = Image::empty();
        image.size = size;
        return Image::add_with_name(&id, image);
    }

    Image::render(id, size, |image| {
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
                &Color::WHITE,
                0,
                true,
            );
        }

        image.flipped_y = true;
        image.channels = 1;
    })
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use refs::set_current_thread_as_main;
    use rtools::Random;
    use serial_test::serial;

    use crate::{render_text, Font};

    #[test]
    #[serial]
    fn text_size() {
        set_current_thread_as_main();
        Font::disable_render();

        for _ in 0..100 {
            let text = String::random();

            let size = u32::random_in(10..100);

            let middle_size = render_text(&text, Font::san_francisco().deref(), size).size;

            let smol_size =
                render_text(&text, Font::san_francisco().deref(), size - u32::random_in(2..6)).size;
            let bigg_size =
                render_text(&text, Font::san_francisco().deref(), size + u32::random_in(2..6)).size;

            assert!(middle_size.width > smol_size.width);
            assert!(middle_size.height > smol_size.height);

            assert!(middle_size.width < bigg_size.width);
            assert!(middle_size.height < bigg_size.height);
        }
    }
}
