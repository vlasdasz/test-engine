use gm::flat::{Point, Rect};
use rtools::{address::Address, Rglica, ToRglica};

use crate::{
    basic::Placer,
    view::{view_data::ViewData, Alert, ViewInternal},
    View, ViewBase, ViewFrame,
};

pub trait ViewSubviews {
    fn superview(&self) -> Rglica<ViewBase>;
    fn subviews(&self) -> &[Box<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Box<dyn View>];
    fn remove_from_superview(&mut self);
    fn remove_subview_at(&mut self, index: usize);
    fn add_view_at(&mut self, point: Point);
    fn remove_all_subviews(&mut self);

    fn add_view<V: 'static + View>(&mut self) -> Rglica<V>;
    fn add_view_with_frame<V: 'static + View>(&mut self, frame: impl Into<Rect>) -> Rglica<V>;
    fn add_boxed(&mut self, view: Box<dyn View>);

    fn alert(&mut self, message: impl ToString);
}

impl<T: ?Sized + View> ViewSubviews for T {
    fn superview(&self) -> Rglica<ViewBase> {
        self.view().superview
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view().subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut().subviews
    }

    fn remove_from_superview(&mut self) {
        let index = self
            .superview()
            .subviews()
            .iter()
            .position(|view| self.address() == view.address())
            .unwrap();

        self.superview().remove_subview_at(index);
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

    fn add_view_with_frame<V: 'static + View>(&mut self, frame: impl Into<Rect>) -> Rglica<V> {
        let mut view = V::boxed();
        view.set_frame(frame.into());
        let result = view.to_rglica();
        self.add_boxed(view);
        result
    }

    fn add_boxed(&mut self, mut view: Box<dyn View>) {
        let result = view.to_rglica();
        view.view_mut().superview = self.view().to_rglica();
        view.view_mut().drawer = self.drawer();
        view.view_mut().placer = Placer::make(result);
        view.setup();
        self.view_mut().subviews.push(view);
    }

    fn alert(&mut self, message: impl ToString) {
        self.root_view().add_view::<Alert>().set_message(message);
    }
}
