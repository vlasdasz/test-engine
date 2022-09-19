use ui::{view, SubView, ViewCallbacks};

use crate::Label;

#[view]
#[derive(Default)]
pub struct LabeledView {
    title: SubView<Label>,
    text:  SubView<Label>,
}

impl LabeledView {
    pub fn set_title(&mut self, label: impl ToString) -> &Self {
        self.title.set_text(label);
        self
    }

    pub fn set_text(&mut self, value: impl ToString) -> &Self {
        self.text.set_text(value);
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.text.clear();
        self
    }
}

impl ViewCallbacks for LabeledView {
    fn setup(&mut self) {
        self.place.all_hor();
    }
}
