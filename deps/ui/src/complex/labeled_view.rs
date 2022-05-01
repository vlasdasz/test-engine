use rtools::Rglica;

use crate::{
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[derive(Default, Debug)]
pub struct LabeledView {
    base:  ViewBase,
    label: Rglica<Label>,
    value: Rglica<Label>,
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
        self.label = self.add_view();
        self.value = self.add_view();
    }

    fn layout(&mut self) {
        self.label.place().left_half();
        self.value.place().right_half();
    }
}

impl View for LabeledView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
