#![cfg(test)]

use test_engine::ViewApp;
use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::GLLabel;

#[view]
struct LabelTestView {
    label: SubView<GLLabel>,
}

impl ViewSetup for LabelTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.place().back();
        self.label.set_text("sokol");
    }
}

#[ignore]
#[test]
fn test_label() {
    ViewApp::<LabelTestView>::start().unwrap()
}
