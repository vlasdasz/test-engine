use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use serde::Deserialize;
use test_engine::{
    dispatch::{from_main, on_main},
    net::{Request, RestAPI},
    refs::Weak,
    ui::{Button, HasText, Label, Setup, Spinner, UIDrawer, ViewFrame, async_link_button, view},
    ui_test::inject_touches,
};

static NOT_REQUESTED: AtomicBool = AtomicBool::new(false);

#[view]
struct RestRequest {
    #[init]
    button: Button,
    label:  Label,
}

impl RestRequest {
    async fn tapped(mut self: Weak<Self>) -> Result<()> {
        #[derive(Debug, Deserialize)]
        struct User {}

        static REQUEST: Request<(), Vec<User>> = Request::new("users");

        let spin = Spinner::lock();

        let users = REQUEST.await?;

        drop(spin);

        assert_eq!(users.len(), 10);

        on_main(move || {
            self.label.set_text(users.len());
            NOT_REQUESTED.store(false, Ordering::Relaxed);
        });

        Ok(())
    }
}

impl Setup for RestRequest {
    fn setup(mut self: Weak<Self>) {
        NOT_REQUESTED.store(true, Ordering::Relaxed);

        RestAPI::init("https://jsonplaceholder.typicode.com/");

        self.button.set_frame((50, 50, 100, 100));
        self.button.set_text("Send");

        self.label.set_frame((200, 50, 100, 100));
        self.label.set_text("Label");

        async_link_button!(self.button, tapped);
    }
}

pub async fn test_rest_request() -> Result<()> {
    let view = UIDrawer::init_test_view::<RestRequest>();

    inject_touches(
        "
            111  63   b
            111  63   e

        ",
    );

    while NOT_REQUESTED.load(Ordering::Relaxed) {}

    let value = from_main(move || view.label.text.clone());

    assert_eq!(value, "10");

    Ok(())
}
