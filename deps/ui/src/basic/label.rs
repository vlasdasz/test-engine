use std::ops::Deref;

use derivative::Derivative;
use gl_image::Image;
use gm::Color;
use rtools::{
    data_manager::{DataManager, Handle},
    Rglica, ToRglica,
};

use crate::{
    basic::label_layout::LabelLayout,
    impl_view, view,
    view::{ViewData, ViewFrame, ViewSubviews},
    Font, ImageView, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:       Handle<Font>,
    text:       String,
    #[derivative(Debug = "ignore")]
    layout:     LabelLayout,
    image_view: Rglica<ImageView>,
    text_color: Color,
}

impl_view!(Label);

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

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn set_text_color(&mut self, _color: impl Into<Color>) -> &mut Self {
        //self.image_view.view_mut().image.color = color.into();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn set_letters(&mut self) {
        if self.text.is_empty() {
            self.image_view.set_image(Default::default());
            return;
        }

        if let Some(image) = Image::handle_with_name(self.text.clone()) {
            self.image_view.set_image(image);
            return;
        }

        self.layout.clear();
        self.layout.set_text(&self.font, &self.text);
        let size = self.layout.size();

        let image = Image::draw(self.text.clone(), size, |image| {
            let drawer = self.drawer();

            let mut content = ViewBase::default();
            content.set_frame(size);

            for glyph in self.layout.glyphs() {
                let image = self.font.glyph_for_char(glyph.parent).image;
                drawer.draw_image(
                    image.deref(),
                    &(
                        glyph.x,
                        size.height - glyph.y - glyph.height as f32 + 10.0,
                        glyph.width,
                        glyph.height,
                    )
                        .into(),
                    &Color::WHITE, // See ui_monochrome shader
                    true,
                );
            }

            image.flipped_y = true;
            image.channels = 1;
            drawer.reset_viewport();
        });

        self.image_view.set_image(image);
    }
}

impl ViewCallbacks for Label {
    fn setup(&mut self) {
        self.text_color = Color::GREEN;
        self.font = Font::default();

        self.image_view = self.add_view();
        self.image_view.new_placer().as_background();

        self.set_letters();
    }
}
