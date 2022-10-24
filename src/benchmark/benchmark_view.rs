use std::string::String;

use test_engine::{
    rtools::Random,
    ui::{layout::Anchor, SubView},
    view, Screen,
};
use tokio::spawn;
use ui::{
    refs::{Own, Strong, ToWeak, Weak},
    UIManager, ViewCallbacks,
};
use ui_views::{Button, Label, LabeledTextField};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct UIDebugView {
    password: SubView<LabeledTextField>,
    login:    SubView<LabeledTextField>,

    button: SubView<Button>,

    label: SubView<Label>,

    back: SubView<Button>,
}

impl UIDebugView {
    async fn on_tap(self: Weak<Self>) {}
}

impl ViewCallbacks for UIDebugView {
    fn setup(&mut self) {
        self.login.place.size(200, 80).center_hor();
        self.login.place.anchor(self.password, Anchor::Bot, 20);
        self.login.set_title("Login:");

        self.password.place.size(200, 40).center();
        self.password.set_title("Password:");

        self.button.place.size(100, 40).center_hor();
        self.button.place.anchor(self.login, Anchor::Bot, 20);

        self.button.on_tap.set(self, move |mut this, _| {
            spawn(async move {
                this.on_tap().await;
            });
        });

        self.back.set_text("Back").place.size(120, 20).b(20).center_hor();

        self.back.on_tap.sub(|_| {
            Screen::current().ui.set_level(Strong::<TestGameLevel>::default());
            UIManager::set_view(Own::<TestGameView>::default());
        });

        let this = self.weak();
        self.label.place.br(10).relative(this, Anchor::Size, 0.4);
        self.label.set_text("Skoggo4");
    }
}
