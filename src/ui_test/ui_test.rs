use std::fmt::Debug;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::{Rglica, ToRglica},
    ui::{basic::Button, impl_view, view, View, ViewBase, ViewCallbacks, ViewFrame, ViewSubviews},
    ui_layer::UILayer,
};

use crate::TestGameView;

#[view]
#[derive(Default, Debug)]
pub struct UITestView {
    container: Rglica<ViewBase>,
    test:      Rglica<ViewBase>,
    back:      Rglica<Button>,
    ui:        Rglica<UILayer>,
}

impl_view!(UITestView);

impl ViewCallbacks for UITestView {
    fn setup(&mut self) {
        self.container = self.add_view();
        self.container.set_frame((200, 200, 280, 280));

        self.test = self.container.add_view();
        self.test.set_frame((100, 100, 100, 100));

        self.test.make_layout(|a| {
            a.top().bottom().offset(40);
            a.left().right().offset(10);
        });

        self.back = self.add_view();
        self.back.set_text("Back").set_frame((120, 20));
        self.back.on_tap.set(self, |this, _| {
            this.ui.set_view::<TestGameView>();
        });
    }

    fn layout(&mut self) {
        self.back.place().bottom_center(20);
    }
}

impl HasLevel for UITestView {}
impl MainView for UITestView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
