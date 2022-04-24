use std::fmt::Debug;

use gl_image::Image;
use gm::{
    flat::{Point, Rect},
    Color,
};
use rtools::{address::Address, data_manager::Handle, Boxed, Rglica, ToRglica};

use crate::{
    basic::Placer,
    complex::{Alert, PathData},
};

mod view_base;
mod view_frame;
mod view_touch;
mod view_touch_internal;

pub use view_base::ViewBase;
pub use view_frame::ViewFrame;
pub use view_touch::ViewTouch;

pub trait View: Boxed + Debug {
    fn setup(&mut self) {}

    fn layout(&mut self) {}

    fn update(&mut self) {}

    fn color(&self) -> Color {
        self.view().color
    }

    fn is_hidden(&self) -> bool {
        self.view().is_hidden
    }

    fn root_view(&self) -> Rglica<ViewBase> {
        let mut root = self.superview();
        loop {
            if root.superview().is_null() {
                return root;
            }
            root = root.superview();
        }
    }

    fn superview(&self) -> Rglica<ViewBase> {
        self.view().superview
    }

    fn super_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.frame();
        }
        self.frame()
    }

    fn super_absolute_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.absolute_frame();
        }
        self.absolute_frame()
    }

    fn add_view_at(&mut self, point: Point) {
        let mut view = ViewBase::dummy();
        view.set_origin(point);
        self.add_boxed(view);
    }

    fn absolute_frame(&self) -> &Rect {
        &self.view().absolute_frame
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut().subviews.clear()
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

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view().subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut().subviews
    }

    fn calculate_frames(&mut self) {
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }

    fn paths(&self) -> Option<&[PathData]> {
        None
    }

    fn image(&self) -> Handle<Image> {
        self.view().image
    }

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}

pub trait ViewTemplates {
    fn set_color(&mut self, color: Color) -> &mut Self;
    fn set_image(&mut self, image: Handle<Image>) -> &mut Self;
    fn set_hidden(&mut self, hidden: bool) -> &mut Self;
    fn add_view<V: 'static + View>(&mut self) -> Rglica<V>;
    fn add_view_with_frame<V: 'static + View>(&mut self, frame: impl Into<Rect>) -> Rglica<V>;
    fn add_boxed(&mut self, view: Box<dyn View>);
    fn alert(&mut self, message: impl ToString);
}

impl<T: ?Sized + View> ViewTemplates for T {
    fn set_color(&mut self, color: Color) -> &mut Self {
        self.view_mut().color = color;
        self
    }

    fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.view_mut().image = image;
        self
    }

    fn set_hidden(&mut self, hidden: bool) -> &mut Self {
        self.view_mut().is_hidden = hidden;
        self
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
        view.view_mut().placer = Placer::make(result);
        view.setup();
        self.view_mut().subviews.push(view);
    }

    fn alert(&mut self, message: impl ToString) {
        self.root_view().add_view::<Alert>().set_message(message);
    }
}
