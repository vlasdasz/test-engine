use std::ops::DerefMut;

use tools::Rglica;

use crate::{init_view_on, View, ViewBase};

#[derive(Default)]
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
        self.frame_mut().size = (120, 120).into();

        self.first = init_view_on(self);
        self.second = init_view_on(self.first.deref_mut());
        self.third = init_view_on(self.second.deref_mut());
        self.forth = init_view_on(self.third.deref_mut());
        self.fifth = init_view_on(self.forth.deref_mut());

        self.first.frame_mut().size = (100, 100).into();
        self.second.frame_mut().size = (90, 90).into();
        self.third.frame_mut().size = (80, 80).into();
        self.forth.frame_mut().size = (70, 70).into();
        self.fifth.frame_mut().size = (60, 60).into();
    }

    fn layout(&mut self) {
        self.first.place().center();
        self.second.place().center();
        self.third.place().center();
        self.forth.place().center();
        self.fifth.place().center();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
