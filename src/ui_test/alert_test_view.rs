use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::{link_button, Alert, Button};

#[view]
struct AlertTestView {
    button: SubView<Button>,
}

impl AlertTestView {
    fn on_button_tap(self: Weak<Self>) {
        Alert::show("Prokpudak prokpudok!");
    }
}

impl ViewSetup for AlertTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Show Alert").place.size(200, 50);
        link_button!(self, button, on_button_tap);
    }
}

impl ViewTest for AlertTestView {
    fn test_size() -> Size
    where Self: Sized {
        (1000, 1000).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<AlertTestView>::start();
}
