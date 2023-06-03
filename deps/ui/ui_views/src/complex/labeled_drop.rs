use refs::{ToWeak, Weak};
use ui::{layout::Anchor, view, Labeled, SubView, TextFieldConstraint, ToLabel, ViewSetup};

use crate::{DropDown, Label};

#[view]
pub struct LabeledDrop {
    title: SubView<Label>,
    drop:  SubView<DropDown>,
}

impl LabeledDrop {
    pub fn set_values(&mut self, values: Vec<String>) {
        self.drop.set_values(values)
    }
}

impl Labeled for LabeledDrop {
    fn text(&self) -> &str {
        self.drop.text()
    }

    fn set_text(&mut self, _: &dyn ToLabel) {
        unreachable!("Dont set text directly to DropDown. Use set_values method")
    }

    fn title(&self) -> &str {
        self.title.text()
    }

    fn set_title(&mut self, title: &dyn ToLabel) {
        self.title.set_text(title.to_label());
    }

    fn set_constraint(&mut self, _cons: Option<TextFieldConstraint>) {}

    fn enable_editing(&mut self) {
        self.drop.enable_editing();
    }

    fn disable_editing(&mut self) {
        self.drop.disable_editing();
    }

    fn labeled(&self) -> Weak<dyn Labeled> {
        (self as &dyn Labeled).weak()
    }
}

impl ViewSetup for LabeledDrop {
    fn setup(self: Weak<Self>) {
        self.title.place.lrt(0).h(10).relative(Anchor::Height, 1.0 / 3.0, self);
        self.drop.place.lrb(0).h(20).relative(Anchor::Height, 2.0 / 3.0, self);
    }
}
