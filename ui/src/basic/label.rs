use derivative::Derivative;
use rtools::Boxed;

use crate::{view_base::ViewBase, Font, ImageView, View};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font: Font,
    text: String,
    base: ViewBase,
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
        self.remove_all_subviews();

        if self.text.is_empty() {
            return;
        }

        let mut last_max_x: f32 = 0.0;
        let mut advance: f32 = 0.0;
        let mut content_size = self.base.frame().size;

        content_size.height = self.font.height;

        let text = self.text.clone();

        for letter in text.chars() {
            if letter == ' ' {
                advance += 10.0;
                continue;
            }

            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::boxed();
            glyph_view.frame_mut().size = glyph.size;
            glyph_view.set_image(glyph.image.clone());

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
