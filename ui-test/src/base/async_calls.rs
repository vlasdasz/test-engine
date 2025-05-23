use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Button, Setup, UI, view},
};

#[view]
struct AsyncCalls {
    #[init]
    button: Button,
}

impl Setup for AsyncCalls {
    fn setup(self: Weak<Self>) {
        self.button.on_tap(|| {});
    }
}

pub async fn _test_async_calls() -> Result<()> {
    UI::init_test_view::<AsyncCalls>().await;

    Ok(())
}
