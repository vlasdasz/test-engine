use gm::Color;
use log::warn;
use refs::Weak;
use rtools::IntoF32;
use text::{render_text, Font};
use ui::{view, SubView, ToLabel, View, ViewCallbacks, ViewFrame, ViewSetup, ViewSubviews};

use crate::ImageView;

#[view]
pub struct Label {
    font:         Weak<Font>,
    text:         String,
    image_view:   SubView<ImageView>,
    text_size:    f32,
    needs_update: bool,
    initial_text: Option<String>,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        let text = text.to_label();
        if text.is_empty() {
            self.image_view.is_hidden = true;
            self.text = text;
            return self;
        }

        self.image_view.is_hidden = false;

        if self.text == text {
            return self;
        }
        self.text = text;
        self.needs_update = true;
        self
    }

    pub fn set_text_size(&mut self, size: impl IntoF32) -> &mut Self {
        let mut size = size.into_f32();
        if size < 0.0 {
            warn!("Label size less than zero: {size}. Will be set to 1.");
            size = 1.0;
        }

        if self.text_size.ne(&size) {
            self.needs_update = true;
        }
        self.text_size = size;

        self
    }

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn pop_letter(&mut self) {
        if !self.text.is_empty() {
            self.text.pop();
            self.needs_update = true;
        }
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.image_view.tint_color = color.into();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn fit_size(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let image = self.image_view.image;

        let size = if image.size.width > self.width() {
            image.size.fit_width(self.width())
        } else if image.size.height > self.height() {
            image.size.fit_height(self.height())
        } else {
            image.size
        };

        self.image_view.set_size(size);
    }

    fn set_letters(&mut self) {
        // Image::free(self.image_view.image);
        let image = render_text(&self.text, &mut self.font, self.text_size);
        self.image_view.image = image;
    }
}

impl ViewSetup for Label {
    fn setup(mut self: Weak<Self>) {
        self.font = Font::helvetica();
        self.set_size((100, 20));
        self.text_size = 32.0;

        debug_assert!(self.text.is_empty());
        self.image_view.place.center();
        self.image_view.is_hidden = true;

        if let Some(text) = self.initial_text.take() {
            self.set_text(text);
        }
    }
}

impl ViewCallbacks for Label {
    fn update(&mut self) {
        if self.needs_update {
            self.set_letters();
            self.needs_update = false;
        }
        self.fit_size();
    }
}

impl From<&String> for Label {
    fn from(value: &String) -> Self {
        Label {
            initial_text: value.to_string().into(),
            ..Default::default()
        }
    }
}

pub trait AddLabel {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self;
}

impl<T: ?Sized + View> AddLabel for T {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self {
        let mut label = self.add_view::<Label>();
        label.place.center().h(20).lr(0);
        label.set_text(text);
        self
    }
}
