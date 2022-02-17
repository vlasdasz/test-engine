use std::ops::DerefMut;

use rtools::Rglica;

use crate::{
    view_base::{add_view, ViewBase},
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
        self.first = add_view(self);
        self.second = add_view(self.first.deref_mut());
        self.third = add_view(self.second.deref_mut());
        self.forth = add_view(self.third.deref_mut());
        self.fifth = add_view(self.forth.deref_mut());
    }

    fn layout(&mut self) {
        self.first.place().background_margin(2);
        self.second.place().background_margin(2);
        self.third.place().background_margin(2);
        self.forth.place().background_margin(2);
        self.fifth.place().background_margin(2);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
