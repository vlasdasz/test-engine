use test_engine::{
    ui::{layout::Anchor, SubView},
    view, Screen,
};
use ui::{
    refs::{dump_ref_stats, Own, Strong, ToWeak},
    UIManager, ViewCallbacks,
};
use ui_views::{Alert, Button, Label, LabeledTextField, MultilineLabel};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct UIDebugView {
    password: SubView<LabeledTextField>,
    login:    SubView<LabeledTextField>,

    label:       SubView<Label>,
    multi_label: SubView<MultilineLabel>,

    alert: SubView<Button>,
    back:  SubView<Button>,

    stats: SubView<Button>,
}

impl ViewCallbacks for UIDebugView {
    fn setup(&mut self) {
        self.login.place.size(200, 80).center_hor();
        self.login.place.anchor(self.password, Anchor::Bot, 20);
        self.login.set_title("Login:");

        self.password.place.size(200, 40).center();
        self.password.set_title("Password:");

        self.back.set_text("Back").place.size(120, 20).b(20).center_hor();

        self.alert
            .set_text("Alert")
            .place
            .same([Anchor::Size, Anchor::X], self.back)
            .anchor(self.back, Anchor::Bot, 20);
        self.alert.on_tap.sub(|_| {
            Alert::show("Sokol");
        });

        self.back.on_tap.sub(|_| {
            Screen::current().ui.set_level(Strong::<TestGameLevel>::default());
            UIManager::set_view(Own::<TestGameView>::default());
        });

        let this = self.weak();

        self.label.place.br(10).relative(Anchor::Size, 0.4, this);
        self.label.set_text("Skoggo4");

        self.multi_label.place.tl(10).same_size(self.label);
        self.multi_label.set_text("Multi Skoggo4");

        self.stats.place.size(100, 20).tr(5);
        self.stats.set_text("Print stats");
        self.stats.on_tap.sub(|_| {
            println!("===========================================================");
            dump_ref_stats();
            println!("===========================================================");
        });
    }
}
