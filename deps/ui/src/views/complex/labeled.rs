use refs::{weak_from_ref, Weak};
use ui_proc::view;

use crate::{view::ViewData, Anchor, InputView, Sub, ViewSetup};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::Label;

#[view]
pub struct Labeled<T: InputView + Default + 'static> {
    label:     Sub<Label>,
    pub input: Sub<T>,
}

impl<T: InputView + Default> ViewSetup for Labeled<T> {
    fn setup(self: Weak<Self>) {
        self.label.place().lrt(0).h(10).relative(Anchor::Height, self, 1.0 / 3.0);
        self.input.place().lrb(0).h(20).relative(Anchor::Height, self, 2.0 / 3.0);
    }
}

impl<T: InputView + Default> InputView for Labeled<T> {
    fn set_title(&mut self, title: &str) {
        self.label.set_text(title);
    }

    fn text(&self) -> &str {
        self.input.text()
    }

    fn enable_editing(&mut self) {
        self.input.enable_editing();
    }

    fn disable_editing(&mut self) {
        self.input.disable_editing();
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self as &dyn InputView)
    }
}
