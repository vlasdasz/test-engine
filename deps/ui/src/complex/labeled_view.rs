use rtools::Rglica;

use crate::{
    view::{ViewFrame, ViewTemplates},
    Label, View, ViewBase,
};

#[derive(Default, Debug)]
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
        self.label = self.add_view();
        self.value = self.add_view();
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
