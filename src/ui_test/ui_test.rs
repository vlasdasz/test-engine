use std::fmt::Debug;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::{Rglica, ToRglica},
    ui::{basic::Button, view, BaseView, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout, ViewSubviews},
    ui_layer::UILayer,
};

use crate::TestGameView;

#[view]
#[derive(Default, Debug)]
pub struct UITestView {
    container: Rglica<BaseView>,
    top_view:  Rglica<BaseView>,
    test:      Rglica<BaseView>,
    back:      Rglica<Button>,
    ui:        Rglica<UILayer>,
}

impl ViewCallbacks for UITestView {
    fn setup(&mut self) {
        self.container = self.add_view();
        self.container.set_frame((200, 200, 280, 280));

        self.top_view = self.container.add_view();
        self.top_view.make_layout(|a| {
            a.left().top().right().val(10);
            a.height(50);
        });

        self.test = self.container.add_view();
        self.test.make_layout(|a| {
            a.top().anchor(self.top_view, 20);
            a.left().right().bottom().val(10);
        });

        self.back = self.add_view();
        self.back.set_text("Back").make_layout(|l| {
            l.width(120).height(20);
            l.bottom().val(20);
            l.center_hor();
        });
        self.back.on_tap.set(self, |this, _| {
            this.ui.set_view::<TestGameView>();
        });
    }
}

impl HasLevel for UITestView {}
impl MainView for UITestView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
