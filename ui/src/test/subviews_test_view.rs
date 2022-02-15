use std::ops::DerefMut;

use rtools::Rglica;

use crate::{
    view_base::{init_view_on, ViewBase},
    View,
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

impl View for SubviewsTestView {
    fn setup(&mut self) {
        self.first = init_view_on(self);
        self.second = init_view_on(self.first.deref_mut());
        self.third = init_view_on(self.second.deref_mut());
        self.forth = init_view_on(self.third.deref_mut());
        self.fifth = init_view_on(self.forth.deref_mut());
    }

    fn layout(&mut self) {
        self.first.place().background_margin(4);
        self.second.place().background_margin(4);
        self.third.place().background_margin(4);
        self.forth.place().background_margin(4);
        self.fifth.place().background_margin(4);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
