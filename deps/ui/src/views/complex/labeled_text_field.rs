use std::fmt::Display;

use refs::{weak_from_ref, Weak};
use ui_proc::view;

use crate::{view::ViewData, Anchor, Labeled, Sub, TextFieldConstraint, ToLabel, ViewSetup};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Label, TextField};

#[view]
pub struct LabeledTextField {
    label:      Sub<Label>,
    text_field: Sub<TextField>,
}

impl Labeled for LabeledTextField {
    fn text(&self) -> &str {
        self.text_field.text()
    }

    fn set_text(&mut self, text: &dyn ToLabel) {
        self.text_field.set_text(text.to_label());
    }

    fn title(&self) -> &str {
        self.label.text()
    }

    fn set_title(&mut self, title: &dyn ToLabel) {
        self.label.set_text(title.to_label());
    }

    fn set_constraint(&mut self, cons: Option<TextFieldConstraint>) {
        self.text_field.constraint = cons;
    }

    fn enable_editing(&mut self) {
        self.text_field.enable_editing();
    }

    fn disable_editing(&mut self) {
        self.text_field.disable_editing();
    }

    fn labeled(&self) -> Weak<dyn Labeled> {
        weak_from_ref(self as &dyn Labeled)
    }
}

impl ViewSetup for LabeledTextField {
    fn setup(self: Weak<Self>) {
        self.label.place().lrt(0).h(10).relative(Anchor::Height, self, 1.0 / 3.0);
        self.text_field.place().lrb(0).h(20).relative(Anchor::Height, self, 2.0 / 3.0);
    }
}

impl Display for LabeledTextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}
