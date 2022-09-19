use gm::Color;
use rtools::data_manager::Handle;
use smart_default::SmartDefault;
use text::{render_text, Font};
use ui::{view, SubView, ViewCallbacks, ViewData};

use crate::ImageView;

#[view]
#[derive(SmartDefault)]
pub struct Label {
    font:       Handle<Font>,
    text:       String,
    image_view: SubView<ImageView>,
    text_color: Color,
    #[default = 64.0]
    size:       f32,
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

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn pop_letter(&mut self) {
        if !self.text.is_empty() {
            self.text.pop();
            self.set_letters();
        }
    }

    pub fn set_text_color(&mut self, _color: impl Into<Color>) -> &mut Self {
        //self.image_view.view_mut().image.color = color.into();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn set_letters(&mut self) {
        let image = render_text(&self.text, &self.font, self.size);
        self.image_view.set_image(image);
    }
}

impl ViewCallbacks for Label {
    fn setup(&mut self) {
        self.text_color = Color::GREEN;
        self.font = Font::default();

        self.image_view.place.as_background();

        self.set_letters();
    }
}
