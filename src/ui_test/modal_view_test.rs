use async_trait::async_trait;
use test_engine::gm::{flat::Size, Color};
use ui::{
    refs::{Own, Weak},
    view, ModalView, SubView, ViewData, ViewSetup, ViewTest,
};
use ui_views::{link_button, Button};

#[view]
struct ModalTestView {
    button: SubView<Button>,
}

impl ViewSetup for ModalTestView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE);
        self.button.set_text("Tap").place.size(100, 20).center();
    }
}

#[async_trait]
impl ModalView for ModalTestView {
    fn modal_size() -> Size {
        (400, 400).into()
    }
}

#[view]
struct ModalViewTestContainer {
    button: SubView<Button>,
}

impl ModalViewTestContainer {
    fn on_tap(self: Weak<Self>) {
        let modal = Own::<ModalTestView>::default();
        modal.show_modally();
    }
}

impl ViewSetup for ModalViewTestContainer {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Tap").place.size(100, 50);
        link_button!(self, button, on_tap);
    }
}

impl ViewTest for ModalViewTestContainer {
    fn test_size() -> Size
    where Self: Sized {
        (1000, 1000).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<ModalViewTestContainer>::start()
}
