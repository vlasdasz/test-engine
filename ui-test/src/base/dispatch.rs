use std::sync::Mutex;

use anyhow::Result;
use test_engine::{
    dispatch::{Task, from_main},
    refs::Weak,
    ui::{Button, HasText, Setup, Spinner, UI, ViewData, link_button, view},
    ui_test::inject_touches,
};
use tokio::sync::oneshot::{Receiver, channel};

static RECEIVER: Mutex<Option<Receiver<()>>> = Mutex::new(None);

#[view]
pub struct AsyncDispatch {
    value:  u64,
    #[init]
    button: Button,
}

impl AsyncDispatch {
    fn tapped(mut self: Weak<Self>) {
        let (se, re) = channel::<()>();

        *RECEIVER.lock().unwrap() = Some(re);

        let spin = Spinner::lock();
        Task::blocking(|| (1..100_000_000).into_iter().sum::<u64>()).callback(move |sum| {
            self.value = sum;

            drop(spin);
            se.send(()).unwrap()
        });
    }
}

impl Setup for AsyncDispatch {
    fn setup(mut self: Weak<Self>) {
        self.value = 5;

        self.button.set_text("Press");
        self.button.place().tl(20).size(100, 100);

        link_button!(self, button, tapped);
    }
}

pub async fn test_dispatch() -> Result<()> {
    let mut view = UI::init_test_view::<AsyncDispatch>().await;

    assert_eq!(view.value, 5);

    from_main(move || {
        view.value = 10;
    })
    .await;

    assert_eq!(view.value, 10);

    inject_touches(
        "
            94   99   b
            94   99   e
        ",
    )
    .await;

    let rc = RECEIVER.lock().unwrap().take().unwrap();

    assert_eq!(view.value, 10);

    rc.await?;

    assert_eq!(view.value, 4999999950000000);

    Ok(())
}
