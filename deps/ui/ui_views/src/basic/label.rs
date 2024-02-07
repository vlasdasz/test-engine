use gm::Color;
use log::warn;
use refs::Weak;
use ui::{view, ToLabel, View, ViewData, ViewSetup, ViewSubviews};

#[view]
pub struct Label {
    pub text: String,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.text = text.to_label();
        self
    }

    pub fn set_text_color(&mut self, _color: Color) -> &mut Self {
        warn!("unimplemented");
        self
    }

    pub fn set_text_size(&mut self, _size: u32) -> &mut Self {
        warn!("unimplemented");
        self
    }
}

impl ViewSetup for Label {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE);
    }
}

pub trait AddLabel {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self;
}

impl<T: ?Sized + View> AddLabel for T {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self {
        let mut label = self.add_view::<Label>();
        label.place().center().h(20).lr(0);
        label.text = text.to_label();
        self
    }
}
