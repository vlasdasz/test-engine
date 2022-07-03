use std::ops::DerefMut;

use gm::flat::{Point, Rect};
use rtools::{Rglica, ToRglica};

use crate::{
    layout::Placer,
    view::{view_data::ViewData, Alert, ViewInternal},
    View, ViewBase, ViewFrame,
};

pub trait ViewSubviews {
    fn superview(&self) -> Rglica<dyn View>;
    fn subviews(&self) -> &[Box<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Box<dyn View>];
    fn remove_from_superview(&mut self);
    fn remove_subview_at(&mut self, index: usize);
    fn add_view_at(&mut self, point: Point);
    fn remove_all_subviews(&mut self);

    fn add_view<V: 'static + View>(&mut self) -> Rglica<V>;
    fn make_view<V: 'static + View>(&mut self, make: impl FnOnce(&mut V)) -> Rglica<V>;
    fn add_view_with_frame<V: 'static + View>(&mut self, frame: impl Into<Rect>) -> Rglica<V>;
    fn add_boxed(&mut self, view: Box<dyn View>);

    fn alert(&mut self, message: impl ToString);
}

impl<T: ?Sized + View> ViewSubviews for T {
    fn superview(&self) -> Rglica<dyn View> {
        self.view().superview
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view().subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut().subviews
    }

    fn remove_from_superview(&mut self) {
        self.root_view().shedule_remove(self.rglica())
    }

    fn remove_subview_at(&mut self, index: usize) {
        self.view_mut().subviews.remove(index);
    }

    fn add_view_at(&mut self, point: Point) {
        let mut view = ViewBase::dummy();
        view.set_origin(point);
        self.add_boxed(view);
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut().subviews.clear()
    }

    fn add_view<V: 'static + View>(&mut self) -> Rglica<V> {
        let view = V::boxed();
        let result = view.to_rglica();
        self.add_boxed(view);
        result
    }

    fn make_view<V: 'static + View>(&mut self, make: impl FnOnce(&mut V)) -> Rglica<V> {
        let view = V::boxed();
        let mut result = view.to_rglica();
        self.add_boxed(view);
        make(result.deref_mut());
        result
    }

    fn add_view_with_frame<V: 'static + View>(&mut self, frame: impl Into<Rect>) -> Rglica<V> {
        let mut view = V::boxed();
        view.set_frame(frame.into());
        let result = view.to_rglica();
        self.add_boxed(view);
        result
    }

    fn add_boxed(&mut self, mut view: Box<dyn View>) {
        view.view_mut().superview = self.rglica();
        view.view_mut().drawer = self.drawer();
        view.view_mut().placer = Placer::make(view.rglica());
        view.view_mut().root_view = self.root_view();
        view.setup();
        self.view_mut().subviews.push(view);
    }

    fn alert(&mut self, message: impl ToString) {
        self.root_view().add_view::<Alert>().set_message(message);
    }
}
