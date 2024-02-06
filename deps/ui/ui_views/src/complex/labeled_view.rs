use refs::Weak;
use ui::{view, SubView, ToLabel, ViewData, ViewSetup};

use crate::GLLabel;

#[view]
pub struct LabeledView {
    title: SubView<GLLabel>,
    text:  SubView<GLLabel>,
}

impl LabeledView {
    pub fn set_title(&mut self, label: impl ToLabel) -> &Self {
        self.title.set_text(label);
        self
    }

    pub fn set_text(&mut self, value: impl ToLabel) -> &Self {
        self.text.set_text(value);
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.text.clear();
        self
    }
}

impl ViewSetup for LabeledView {
    fn setup(self: Weak<Self>) {
        self.place().all_hor();
    }
}
