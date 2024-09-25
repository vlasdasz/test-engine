use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, KeyboardView, Setup, ViewData, UI},
    App,
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
    let _view = UI::init_test_view::<KeyboardViewTest>().await;

    App::set_window_size((800, 400)).await;

    //  record_ui_test().await;

    debug!("Keyboard view: OK");

    Ok(())
}
