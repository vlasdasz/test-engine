use anyhow::Result;
use log::debug;
use test_engine::{
    AppRunner,
    refs::Weak,
    ui::{KeyboardView, Setup, ViewData, view},
    ui_test::UITest,
};

#[view]
struct KeyboardViewTest {
    #[init]
    keyboard: KeyboardView,
}

impl Setup for KeyboardViewTest {
    fn setup(self: Weak<Self>) {
        self.keyboard.place().back();
    }
}

pub async fn test_keyboard_view() -> Result<()> {
    let _view = UITest::init::<KeyboardViewTest>();

    AppRunner::set_window_size((800, 400));

    //  record_ui_test().await;

    debug!("Keyboard view: OK");

    Ok(())
}
