use anyhow::Result;
use log::debug;
use test_engine::{
    AppRunner,
    refs::Weak,
    ui::{KeyboardView, Setup, UI, ViewData, view},
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
    let _view = UI::init_test_view::<KeyboardViewTest>();

    AppRunner::set_window_size((800, 400));

    //  record_ui_test().await;

    debug!("Keyboard view: OK");

    Ok(())
}
