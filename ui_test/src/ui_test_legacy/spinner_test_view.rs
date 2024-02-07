use rtools::sleep;
use ui::{refs::Weak, view, SubView, ViewSetup};
use ui_views::{async_link_button, touch_test_view::TouchTestView, Button, Spinner};

#[view]
struct SpinnerTestView {
    button: SubView<Button>,
    test:   SubView<TouchTestView>,
}

impl SpinnerTestView {
    async fn tap(self: Weak<Self>) {
        Spinner::start();
        sleep(3);
        Spinner::stop();
    }
}

impl ViewSetup for SpinnerTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Spin");
        async_link_button!(self, button, tap);
    }
}

#[ignore]
#[test]
fn test() {
    old_engine::ViewApp::<SpinnerTestView>::start().unwrap()
}
