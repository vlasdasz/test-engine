use test_engine::{
    ui::{layout::Anchor, SubView},
    view, Screen,
};
use ui::{
    refs::{Own, Strong, ToWeak},
    UIManager, ViewCallbacks,
};
use ui_views::{Button, Label, LabeledTextField};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct UIDebugView {
    password: SubView<LabeledTextField>,
    login:    SubView<LabeledTextField>,

    label: SubView<Label>,

    back: SubView<Button>,
}

impl ViewCallbacks for UIDebugView {
    fn setup(&mut self) {
        self.login.place.size(200, 80).center_hor();
        self.login.place.anchor(self.password, Anchor::Bot, 20);
        self.login.set_title("Login:");

        self.password.place.size(200, 40).center();
        self.password.set_title("Password:");

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
