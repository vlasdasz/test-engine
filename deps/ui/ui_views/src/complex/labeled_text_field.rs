use refs::Weak;
use ui::{layout::Anchor, view, SubView, ViewSetup};

use crate::{Label, TextField};

#[view]
pub struct LabeledTextField {
    label:      SubView<Label>,
    text_field: SubView<TextField>,
}

impl LabeledTextField {
    pub fn text(&self) -> &str {
        self.text_field.text()
    }

    pub fn set_title(&mut self, title: impl ToString) -> &mut Self {
        self.label.set_text(title);
        self
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.text_field.set_text(text);
        self
    }

    pub fn text_field(&self) -> Weak<TextField> {
        self.text_field.weak()
    }

    pub fn enable_editing(&mut self) -> &mut Self {
        self.text_field.enable_editing();
        self
    }

    pub fn disable_editing(&mut self) -> &mut Self {
        self.text_field.disable_editing();
        self
    }
}

impl ViewSetup for LabeledTextField {
    fn setup(self: Weak<Self>) {
        self.label.place.lrt(0).h(10).relative(Anchor::Height, 1.0 / 3.0, self);
        self.text_field.place.lrb(0).h(20).relative(Anchor::Height, 2.0 / 3.0, self);
    }
}
