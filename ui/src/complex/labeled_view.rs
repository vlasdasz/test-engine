use rtools::Rglica;

use crate::{
    view_base::{init_view_on, ViewBase},
    Label, View,
};

#[derive(Default)]
pub struct LabeledView {
    base:  ViewBase,
    label: Rglica<Label>,
    value: Rglica<Label>,
}

impl LabeledView {
    pub fn set_label(&mut self, label: impl ToString) {
        self.label.set_text(label)
    }

    pub fn set_value(&mut self, value: impl ToString) {
        self.value.set_text(value)
    }

    pub fn clear(&mut self) {
        self.value.clear()
    }
}

impl View for LabeledView {
    fn setup(&mut self) {
        self.label = init_view_on(self);
        self.value = init_view_on(self);
    }

    fn layout(&mut self) {
        self.label.place().left_half();
        self.value.place().right_half();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
