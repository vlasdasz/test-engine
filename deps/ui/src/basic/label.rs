use std::ops::Deref;

use derivative::Derivative;
use gl_image::Image;
use gl_wrapper::GLWrapper;
use gm::Color;
use rtools::{data_manager::DataManager, Rglica, ToRglica};

use crate::{
    basic::label_layout::LabelLayout,
    view::{ViewData, ViewFrame, ViewSubviews},
    Font, ImageView, View, ViewBase,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:   Font,
    text:   String,
    base:   ViewBase,
    #[derivative(Debug = "ignore")]
    layout: LabelLayout,
    image:  Rglica<ImageView>,
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
        if self.text.is_empty() {
            self.image.set_image(Default::default());
            return;
        }

        if let Some(image) = Image::handle_with_name(&self.text) {
            self.image.set_image(image);
            return;
        }

        self.layout.clear();
        self.layout.set_text(&self.font, &self.text);
        let size = self.layout.size();

        let mut this = self.to_rglica();
        let image = Image::draw(&self.text, size, move |image| {
            //            GLWrapper::set_clear_color(Color::WHITE);
            GLWrapper::clear();

            let mut content = ViewBase::default();
            content.set_frame(size);
            let mut fe = this;
            let glyphs = this.layout.glyphs();

            for glyph in glyphs {
                let image = fe.font.glyph_for_char(glyph.parent).image;
                fe.drawer().draw_image(
                    image.deref(),
                    &(glyph.x, glyph.y, glyph.width, glyph.height).into(),
                    Color::BLACK,
                    true,
                );
            }
            image.flipped_y = true;
            //      GLWrapper::set_clear_color(Color::GRAY);
            fe.drawer().reset_viewport();
        });

        self.image.set_image(image);
    }
}

impl View for Label {
    fn setup(&mut self) {
        self.image = self.add_view();
        self.set_letters();
    }

    fn layout(&mut self) {
        self.image.place().as_background();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
