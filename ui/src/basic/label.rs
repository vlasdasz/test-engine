use tools::Boxed;

use crate::{Font, ImageView, View, ViewBase};

#[derive(Default)]
pub struct Label {
    font:  Font,
    _text: String,
    base:  ViewBase,
}

impl Label {
    pub fn text(&self) -> &str {
        &self._text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self._text = text.into();
        self.set_letters();
    }

    fn set_letters(&mut self) {
        self.remove_all_subviews();

        if self._text.is_empty() {
            return;
        }

        let mut last_max_x: f32 = 0.0;
        let mut advance: f32 = 0.0;
        let mut content_size = self.base.frame().size;

        content_size.height = self.font.height;

        let text = self._text.clone();

        for letter in text.chars() {
            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::boxed();
            glyph_view.frame_mut().size = glyph.size;
            glyph_view.image = glyph.image.clone();

            glyph_view.set_frame(
                (
                    advance,// + glyph.bearing.x,
                    content_size.height - glyph.bearing.y + self.font.baseline_shift,
                    glyph.size.width,
                    glyph.size.height,
                )
                    .into(),
            );

            last_max_x = glyph_view.frame().max_x();

            advance += glyph.advance as f32;

            self.add_subview(glyph_view);
        }

        content_size.width = last_max_x;

        let frame = (
            self.view().frame().origin.x,
            self.view().frame().origin.y,
            content_size.width,
            content_size.height,
        )
            .into();

        self.set_frame(frame);
    }
}

impl View for Label {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
