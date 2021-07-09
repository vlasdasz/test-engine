use crate::gm::Rect;

use crate::ui::view::View;
use crate::ui::{Font, ImageView, ViewBase};
use std::any::Any;
use tools::refs::make_shared;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct Label {
    pub font: Font,
    text: String,
    base: ViewBase,
}

impl Label {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.into()
    }
}

impl AsAny for Label {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasNew for Label {
    fn new() -> Self {
        Self {
            font: Font::blank(),
            text: String::new(),
            base: ViewBase::new(),
        }
    }
}

impl View for Label {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }

    fn ptr(&self) -> *const dyn View {
        self as *const dyn View
    }

    fn update(&mut self) {
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
            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::from_rect(glyph.size.into());
            glyph_view.image = glyph.image;

            glyph_view.set_frame(Rect::make(
                advance + glyph.bearing.x,
                content_size.height - glyph.bearing.y + self.font.baseline_shift,
                glyph.size.width,
                glyph.size.height,
            ));

            last_max_x = glyph_view.frame().max_x();

            advance += glyph.advance as f32;

            self.add_subview(make_shared(glyph_view));
        }

        content_size.width = last_max_x;

        let frame = Rect::make(
            self.view().frame().origin.x,
            self.view().frame().origin.y,
            content_size.width,
            content_size.height,
        );

        self.set_frame(frame);
    }
}
