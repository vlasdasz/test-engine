use derivative::Derivative;
use rtools::{Boxed, Rglica};

use crate::{
    view_base::{add_view, ViewBase},
    Font, ImageView, View,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:    Font,
    text:    String,
    base:    ViewBase,
    content: Rglica<ViewBase>,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) {
        self.text = text.to_string();
        self.set_letters();
    }

    pub fn clear(&mut self) {
        self.set_text("")
    }

    fn set_letters(&mut self) {
        self.content.remove_all_subviews();

        if self.text.is_empty() {
            return;
        }

        let mut last_max_x: f32 = 0.0;
        let mut advance: f32 = 0.0;
        let mut content_size = self.base.frame().size;

        content_size.height = self.font.height / 2.0;

        let text = self.text.clone();

        for letter in text.chars() {
            if letter == ' ' {
                advance += 10.0;
                continue;
            }

            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::boxed();
            glyph_view.frame_mut().size = glyph.size;
            glyph_view.set_image(glyph.image);

            glyph_view.set_frame(
                (
                    advance, // + glyph.bearing.x,
                    content_size.height - glyph.bearing.y + self.font.baseline_shift,
                    glyph.size.width,
                    glyph.size.height,
                )
                    .into(),
            );

            last_max_x = glyph_view.frame().max_x();

            advance += glyph.advance as f32;

            self.content.add_subview(glyph_view);
        }

        content_size.width = last_max_x;

        self.content.frame_mut().size = content_size;
    }
}

impl View for Label {
    fn setup(&mut self) {
        self.content = add_view(self);
    }

    fn layout(&mut self) {
        self.content.place().center();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
