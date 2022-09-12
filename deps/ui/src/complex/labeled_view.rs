use rtools::{Rglica, ToRglica};

use crate::{
    view,
    view::{ViewLayout, ViewSubviews},
    Label, SubView, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default)]
pub struct LabeledView {
    label: SubView<Label>,
    value: SubView<Label>,
}

impl LabeledView {
    pub fn set_label(&mut self, label: impl ToString) -> &Self {
        self.label.set_text(label);
        self
    }

    pub fn set_value(&mut self, value: impl ToString) -> &Self {
        self.value.set_text(value);
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.value.clear();
        self
    }
}

impl ViewCallbacks for LabeledView {
    fn setup(&mut self) {
        self.place().all_hor();
    }
}
