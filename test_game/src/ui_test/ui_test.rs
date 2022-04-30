use std::fmt::Debug;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::Rglica,
    ui::{View, ViewBase},
    ui_layer::UILayer,
};

#[derive(Default, Debug)]
pub struct UITestView {
    view: ViewBase,
    ui:   Rglica<UILayer>,
}

impl View for UITestView {
    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}

impl HasLevel for UITestView {}

impl MainView for UITestView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
