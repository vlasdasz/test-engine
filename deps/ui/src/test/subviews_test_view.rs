use rtools::Rglica;

use crate::{
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks,
};

#[derive(Default, Debug)]
pub struct SubviewsTestView {
    base:   ViewBase,
    first:  Rglica<ViewBase>,
    second: Rglica<ViewBase>,
    third:  Rglica<ViewBase>,
    forth:  Rglica<ViewBase>,
    fifth:  Rglica<ViewBase>,
}

impl ViewCallbacks for SubviewsTestView {
    fn setup(&mut self) {
        self.first = self.add_view();
        self.second = self.first.add_view();
        self.third = self.second.add_view();
        self.forth = self.third.add_view();
        self.fifth = self.forth.add_view();
    }

    fn layout(&mut self) {
        self.first.place().background_margin(2);
        self.second.place().background_margin(2);
        self.third.place().background_margin(2);
        self.forth.place().background_margin(2);
        self.fifth.place().background_margin(2);
    }
}

impl View for SubviewsTestView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
