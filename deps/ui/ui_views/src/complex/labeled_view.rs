use refs::Weak;
use ui::{view, Sub, ToLabel, ViewData, ViewSetup};
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::Label;

#[view]
pub struct LabeledView {
    title: Sub<Label>,
    text:  Sub<Label>,
}

impl LabeledView {
    pub fn set_title(&mut self, label: impl ToLabel) -> &Self {
        self.title.text = label.to_label();
        self
    }

    pub fn set_text(&mut self, value: impl ToLabel) -> &Self {
        self.text.text = value.to_label();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.text = Default::default();
        self
    }
}

impl ViewSetup for LabeledView {
    fn setup(self: Weak<Self>) {
        self.place().all_hor();
    }
}
