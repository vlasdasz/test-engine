use test_engine::{gm::flat::Size, on_main};
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::{async_link_button, Alert, Button, Label};

#[view]
struct AlertTestView {
    button: SubView<Button>,
    label:  SubView<Label>,
}

impl AlertTestView {
    async fn on_button_tap(mut self: Weak<Self>) {
        let res = Alert::ask("Prokpudak prokpudok!").await;
        on_main(move || {
            self.label.set_text(res);
        });
    }
}

impl ViewSetup for AlertTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Show Alert").place.size(200, 50);
        self.label.place.size(200, 50).tr(0);
        async_link_button!(self, button, on_button_tap);
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
