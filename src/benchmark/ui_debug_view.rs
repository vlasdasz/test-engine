use test_engine::{
    ui::{layout::Anchor, SubView},
    view, Screen,
};
use ui::{
    refs::{dump_ref_stats, Own, Weak},
    Labeled, UIManager, ViewSetup,
};
use ui_views::{Button, Label, LabeledTextField, MultilineLabel};

use crate::{
    test_game::{TestGameLevel, TestGameView},
    ui_test::CollectionTestView,
};

#[view]
pub struct UIDebugView {
    password: SubView<LabeledTextField>,
    login:    SubView<LabeledTextField>,

    label:       SubView<Label>,
    multi_label: SubView<MultilineLabel>,

    test_game:  SubView<Button>,
    collection: SubView<Button>,

    stats: SubView<Button>,
}

impl ViewSetup for UIDebugView {
    fn setup(mut self: Weak<Self>) {
        self.login.place.size(200, 80).center_hor();

        self.login.place.anchor(self.password, Anchor::Bot, 20);
        self.login.set_title(&"Login:");

        self.password.place.size(200, 40).center();
        self.password.set_title(&"Password:");

        self.test_game.set_text("Test Game").place.size(120, 20).b(20).center_hor();

        self.test_game.on_tap.sub(|| {
            Screen::current().ui.set_level(Own::<TestGameLevel>::default());
            UIManager::set_view(Own::<TestGameView>::default());
        });

        self.collection.set_text("Collection");
        self.collection.place.above(self.test_game, 20);
        self.collection.on_tap.sub(|| {
            UIManager::set_view(Own::<CollectionTestView>::default());
        });

        self.label.place.br(10).relative(Anchor::Size, 0.4, self);
        self.label.set_text_size(64);
        self.label.set_text("Skoggo4");

        self.multi_label.place.tl(10).same_size(self.label);
        self.multi_label.set_text("Multi Skoggo4 Ultra Boggo4 Sopokokt4ek smeorglil4ek");

        self.stats.place.size(100, 20).tr(5);
        self.stats.set_text("Print stats");
        self.stats.on_tap.sub(|| {
            println!("===========================================================");
            dump_ref_stats();
            println!("===========================================================");
        });
    }
}
