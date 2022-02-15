use rtools::Rglica;

use crate::{
    test::subviews_test_view::SubviewsTestView,
    view_base::{init_view_on, ViewBase},
    Label, View,
};

#[derive(Default, Debug)]
pub struct TestView {
    base:     ViewBase,
    label:    Rglica<Label>,
    subviews: Rglica<SubviewsTestView>,
}

impl View for TestView {
    fn setup(&mut self) {
        self.label = init_view_on(self);
        self.label.set_text("Hello label!");

        self.subviews = init_view_on(self);
    }

    fn layout(&mut self) {
        self.place().all_vertically()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
