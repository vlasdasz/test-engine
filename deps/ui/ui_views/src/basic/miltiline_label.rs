#![allow(unused_imports)]
#![allow(dead_code)]

use gm::Color;
use refs::Weak;
use rtools::data_manager::Handle;
use smart_default::SmartDefault;
use text::{render_text, text_size, Font};
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, ViewSubviews};

use crate::ImageView;

#[view]
#[derive(SmartDefault)]
pub struct MultilineLabel {
    #[default(Font::san_francisco())]
    font:   Handle<Font>,
    text:   String,
    images: Vec<(String, Weak<ImageView>)>,
    #[default = 32.0]
    size:   f32,
}

impl MultilineLabel {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn pop_letter(&mut self) {
        if !self.text.is_empty() {
            self.text.pop();
        }
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn set_letters(&mut self) {
        self.remove_all_subviews();

        let size = text_size(&self.text, &self.font, self.size);

        if size.width < self.width() {
            let image = render_text(&self.text, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
        } else {
            let text = self.text.clone();
            let (a, b) = text.split_at(self.text.len() / 2);
            let image = render_text(a, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
            let image = render_text(b, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
        }
    }
}

impl ViewCallbacks for MultilineLabel {
    fn setup(&mut self) {
        // self.text_color = Color::GREEN;kjlfdgfsd
        // self.font = Font::san_francisco();
        //
        // self.image_view.place.center();
        //
        self.set_letters();
        self.place.all_ver();
    }

    fn update(&mut self) {
        self.set_letters();
        // if self.needs_update {
        //     self.set_letters();
        //     self.needs_update = false;
        // }
        // self.fit_size();
    }
}
