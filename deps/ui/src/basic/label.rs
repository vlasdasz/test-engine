use derivative::Derivative;
use rtools::Rglica;

use crate::{
    basic::label_layout::LabelLayout,
    view::{ViewData, ViewFrame, ViewSubviews},
    Font, ImageView, View, ViewBase,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:    Font,
    text:    String,
    base:    ViewBase,
    content: Rglica<ViewBase>,
    #[derivative(Debug = "ignore")]
    layout:  LabelLayout,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) {
        let text = text.to_string();
        if self.text == text {
            return;
        }
        self.text = text;
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

        self.layout.set_text(&self.font, &self.text);

        dbg!(self.layout.size());

        dbg!(self.drawer());

        for glyph in self.layout.glyphs() {
            let mut view = self.content.add_view::<ImageView>();
            view.set_frame((glyph.x, glyph.y, glyph.width, glyph.height));
            view.set_image(self.font.glyph_for_char(glyph.parent).image);
        }
    }
}

impl View for Label {
    fn setup(&mut self) {
        self.content = self.add_view();
        self.set_letters();
    }

    fn layout(&mut self) {
        self.content.place().as_background()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
