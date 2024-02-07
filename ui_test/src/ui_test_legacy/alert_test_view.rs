use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::{link_button, touch_test_view::TouchTestView, Alert, Button, Label};

#[view]
struct AlertTestView {
    test:   SubView<TouchTestView>,
    button: SubView<Button>,
    label:  SubView<Label>,
}

impl AlertTestView {
    fn on_button_tap(self: Weak<Self>) {
        Alert::show("Prokpudak prokpudok!!");
    }
}

impl ViewSetup for AlertTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Show Alert").place().size(200, 50);
        self.label.place().size(200, 50).tr(0);
        link_button!(self, button, on_button_tap);
    }
}

#[ignore]
#[test]
fn test() {
    old_engine::ViewApp::<AlertTestView>::start().unwrap()
}
