use std::fmt::Debug;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::{Rglica, StaticStorage},
    ui::{
        basic::{label::DebugLabel, Button},
        impl_view, view, Label, View, ViewBase, ViewCallbacks, ViewFrame, ViewSubviews,
    },
    ui_layer::UILayer,
};

use crate::TestGameView;

#[view]
#[derive(Default, Debug)]
pub struct UITestView {
    label: Rglica<Label>,
    back:  Rglica<Button>,
    ui:    Rglica<UILayer>,
}

impl_view!(UITestView);

impl ViewCallbacks for UITestView {
    fn setup(&mut self) {
        DebugLabel::set(true);

        self.label = self.add_view();
        self.label
            .set_text("Test Text aa ..324234;dfl*#($U#(*&$*(@#")
            .set_frame((100, 100));

        self.back = self.add_view();
        self.back.set_text("Back").set_frame((120, 20));
        self.back.on_tap.set(self, |this, _| {
            DebugLabel::set(false);
            this.ui.set_view::<TestGameView>();
        });
    }

    fn layout(&mut self) {
        self.label.place().center();
        self.back.place().bottom_center(20);
    }
}

impl HasLevel for UITestView {}
impl MainView for UITestView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
