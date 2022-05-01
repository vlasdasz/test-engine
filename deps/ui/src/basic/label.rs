use std::ops::Deref;

use derivative::Derivative;
use gl_image::Image;
use gm::Color;
use rtools::{data_manager::DataManager, static_storage, Rglica, ToRglica};

use crate::{
    basic::label_layout::LabelLayout,
    view,
    view::{ViewData, ViewFrame, ViewSubviews},
    Font, ImageView, View, ViewBase, ViewCallbacks,
};

static_storage!(DebugLabel, bool, false);

#[view]
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:   Font,
    text:   String,
    #[derivative(Debug = "ignore")]
    layout: LabelLayout,
    image:  Rglica<ImageView>,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        let text = text.to_string();
        if self.text == text {
            return self;
        }
        self.text = text;
        self.set_letters();
        self
    }

    pub fn clear(&mut self) -> &Self {
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
            let mut content = ViewBase::default();
            content.set_frame(size);
            let mut fe = this;
            let glyphs = this.layout.glyphs();

            for glyph in glyphs {
                let image = fe.font.glyph_for_char(glyph.parent).image;
                fe.drawer().draw_image(
                    image.deref(),
                    &(
                        glyph.x,
                        size.height - glyph.y - glyph.height as f32 + 10.0,
                        glyph.width,
                        glyph.height,
                    )
                        .into(),
                    Color::WHITE,
                    true,
                );
            }

            image.flipped_y = true;
            image.channels = 1;
            fe.drawer().reset_viewport();
        });

        self.image.set_image(image);
    }
}

impl ViewCallbacks for Label {
    fn setup(&mut self) {
        self.image = self.add_view();
        self.set_letters();
    }

    fn layout(&mut self) {
        self.image.place().as_background();
        if *DebugLabel::get() {
            dbg!(self.image.frame());
        }
    }
}

impl View for Label {
    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}
