use test_engine::{
    ui::{layout::Anchor, SubView},
    ui_layer::UILayer,
    view,
};
use ui::{
    refs::{dump_ref_stats, Own, Weak},
    Labeled, ViewData, ViewSetup,
};
use ui_views::{Button, GLLabel, LabeledTextField, MultilineLabel};

use crate::test_game::TestGameLevel;

#[view]
pub struct UIDebugView {
    password: SubView<LabeledTextField>,
    login:    SubView<LabeledTextField>,

    label:       SubView<GLLabel>,
    multi_label: SubView<MultilineLabel>,

    test_game:  SubView<Button>,
    collection: SubView<Button>,

    stats: SubView<Button>,
}

impl ViewSetup for UIDebugView {
    fn setup(mut self: Weak<Self>) {
        self.login.place().size(200, 80).center_x();

        self.login.place().anchor(Anchor::Bot, self.password, 20);
        self.login.set_title(&"Login:");

        self.password.place().size(200, 40).center();
        self.password.set_title(&"Password:");

        self.test_game.set_text("Test Game").place().size(120, 20).b(20).center_x();

        self.test_game.on_tap(|| {
            UILayer::set_level(Own::<TestGameLevel>::default());
            // UIManager::set_view(Own::<TestGameView>::default());
        });

        self.collection.set_text("Collection");
        self.collection.place().above(self.test_game, 20);
        self.collection.on_tap(|| {
            // UIManager::set_view(Own::<CollectionTestView>::default());
        });

        self.label.place().br(10).relative(Anchor::Size, self, 0.4);
        self.label.set_text_size(64);
        self.label.set_text("Skoggo4");

        self.multi_label.place().tl(10).same_size(self.label);
        self.multi_label.set_text("Multi Skoggo4 Ultra Boggo4 Sopokokt4ek smeorglil4ek");

        self.stats.place().size(100, 20).tr(5);
        self.stats.set_text("Print stats");
        self.stats.on_tap(|| {
            println!("===========================================================");
            dump_ref_stats();
            println!("===========================================================");
        });
    }
}
