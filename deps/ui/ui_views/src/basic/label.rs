use gm::Color;
use refs::Weak;
use ui::{view, ToLabel, View, ViewData, ViewSetup, ViewSubviews};

#[view]
pub struct Label {
    pub text: String,
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
