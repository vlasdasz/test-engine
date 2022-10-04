use refs::ToWeak;
use ui::{layout::Anchor, view, SubView, ViewCallbacks};

use crate::{Label, TextField};

#[view]
#[derive(Default)]
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
}

impl ViewCallbacks for LabeledTextField {
    fn setup(&mut self) {
        let this = self.weak();
        self.label.place.lrt(0).h(10).relative(this, Anchor::Height, 1.0 / 3.0);
        self.text_field.place.lrb(0).h(20).relative(this, Anchor::Height, 2.0 / 3.0);
    }
}
