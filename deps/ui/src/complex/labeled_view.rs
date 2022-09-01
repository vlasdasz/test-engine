use rtools::{Rglica, ToRglica};

use crate::{impl_view, view, view::ViewSubviews, Label, View, ViewBase, ViewCallbacks};

#[view]
#[derive(Default, Debug)]
pub struct LabeledView {
    label: Rglica<Label>,
    value: Rglica<Label>,
}

impl_view!(LabeledView);

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
        self.label = self.add_view();
        self.value = self.add_view();
    }
}
