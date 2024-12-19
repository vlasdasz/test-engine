use std::sync::atomic::{AtomicU16, Ordering};

use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{Button, HasText, Setup, UI, ViewData, view},
    ui_test::inject_touches,
};

static COUNTER: AtomicU16 = AtomicU16::new(0);

#[view]
struct InjectTouchTestView {
    #[init]
    button: Button,
}

impl Setup for InjectTouchTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back();
        self.button.set_text("bress");
        self.button.on_tap(|| COUNTER.fetch_add(1, Ordering::Relaxed));
    }
}

pub async fn test_inject_touch() -> Result<()> {
    COUNTER.store(0, Ordering::Relaxed);

    UI::init_test_view::<InjectTouchTestView>().await;

    let mut touches = String::new();

    for _ in 0..100 {
        touches += r"
            5  5  b
            5  5  e
    ";
    }

    inject_touches(touches).await;

    assert_eq!(COUNTER.load(Ordering::Relaxed), 100);

    for _ in 0..10 {
        inject_touches(
            r"
            5  5  b
            5  5  e
    ",
        )
        .await;
    }

    assert_eq!(COUNTER.load(Ordering::Relaxed), 110);

    debug!("Inject touch test: OK");

    Ok(())
}
