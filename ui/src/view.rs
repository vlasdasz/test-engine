use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{Color, Rect};
use tools::{rglica::ToRglica, Address, Boxed, Event, Rglica};

use crate::{basic::Placer, complex::PathData, input::Touch};

pub trait View: Boxed {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn layout(&mut self) {}

    fn color(&self) -> &Color {
        &self.view()._color
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut()._color = color
    }

    fn superview(&self) -> Rglica<dyn View> {
        self.view()._superview.clone()
    }

    fn super_frame(&self) -> &Rect {
        if self.view()._superview.is_ok() {
            return self.view()._superview.frame();
        }
        self.frame()
    }

    fn super_absolute_frame(&self) -> &Rect {
        if self.view()._superview.is_ok() {
            return self.view()._superview.absolute_frame();
        }
        self.absolute_frame()
    }

    fn frame(&self) -> &Rect {
        &self.view()._frame
    }

    fn frame_mut(&mut self) -> &mut Rect {
        &mut self.view_mut()._frame
    }

    fn set_frame(&mut self, rect: Rect) {
        self.view_mut()._frame = rect
    }

    // compiler panic =\
    // fn width(&self) -> f32 {
    //     self.frame().size.width
    // }
    //
    // fn height(&self) -> f32 {
    //     self.frame().size.height
    // }

    fn absolute_frame(&self) -> &Rect {
        &self.view()._absolute_frame
    }

    fn add_subview(&mut self, mut view: Box<dyn View>) {
        view.view_mut()._superview = Rglica::from_ref(self.view());
        view.view_mut()._placer = Placer::make(view.deref_mut());
        view.setup();
        self.view_mut()._subviews.push(view);
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._subviews.clear()
    }

    fn remove_from_superview(&mut self) {
        let index = self
            .superview()
            .subviews()
            .iter()
            .position(|view| view.deref().address() == (&self).address())
            .unwrap();

        self.superview().remove_subview_at(index);
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view()._subviews
    }

    fn remove_subview_at(&mut self, index: usize) {
        self.view_mut()._subviews.remove(index);
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut()._subviews
    }

    fn calculate_absolute_frame(&mut self) {
        let view = self.view_mut();
        view._absolute_frame = view._frame;
        view._absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_absolute_frame();
        }
    }

    fn enable_touch(&mut self) {
        self.view_mut()._touch_enabled = true
    }

    fn touch_enabled(&self) -> bool {
        self.view()._touch_enabled
    }

    fn handle_touch(&mut self, touch: &Touch) {
        self.view_mut()._on_touch.trigger(*touch);
    }

    fn touch_id(&self) -> u64 {
        *self.view()._touch_id.borrow()
    }

    fn set_touch_id(&self, id: u64) {
        *self.view()._touch_id.borrow_mut() = id;
    }

    fn on_touch(&mut self) -> &mut Event<Touch> {
        &mut self.view_mut()._on_touch
    }

    fn check_touch(&mut self, touch: &mut Touch) -> bool {
        if self.touch_enabled() {
            if touch.is_moved() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.handle_touch(touch);
                return true;
            }

            if touch.is_moved() {
                return false;
            }

            if touch.is_ended() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(0);
                self.handle_touch(touch);
                return true;
            }

            if self.absolute_frame().contains(&touch.position) {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(touch.id);
                self.handle_touch(touch);
                return true;
            }
        }

        for view in self.subviews_mut() {
            if view.check_touch(touch) {
                return true;
            }
        }

        false
    }

    fn paths(&self) -> Option<&[PathData]> {
        None
    }

    fn image(&self) -> Option<Image> {
        None
    }

    fn place(&mut self) -> &mut Placer {
        &mut self.view_mut()._placer
    }

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;

    fn with_frame(frame: Rect) -> Box<Self>
    where
        Self: Sized,
    {
        let mut new = Self::boxed();
        new.set_frame(frame);
        new
    }
}

#[derive(Default)]
pub struct ViewBase {
    _color:         Color,
    _touch_enabled: bool,

    _frame:          Rect,
    _absolute_frame: Rect,

    _superview: Rglica<dyn View>,

    _subviews: Vec<Box<dyn View>>,

    _on_touch: Event<Touch>,
    _touch_id: RefCell<u64>,

    _placer: Placer,
}

impl ViewBase {
    pub fn make_view<T: 'static + View>(&mut self) -> Rglica<T> {
        let view = T::boxed();
        let rglica = view.to_rglica();
        self.add_subview(view);
        rglica
    }

    pub fn make_view_with<T: 'static + View>(&mut self, frame: Rect) -> Rglica<T> {
        let view = T::with_frame(frame);
        let rglica = view.to_rglica();
        self.add_subview(view);
        rglica
    }
}

pub fn init_view_on<T: 'static + View>(view: &mut dyn View) -> Rglica<T> {
    view.view_mut().make_view()
}

pub fn init_view_with_frame<T: 'static + View>(frame: Rect, view: &mut dyn View) -> Rglica<T> {
    view.view_mut().make_view_with(frame)
}

pub fn make_view_on<T: 'static + View>(
    view: &mut dyn View,
    make: impl FnOnce(&mut T),
) -> Rglica<T> {
    let new = T::boxed();
    let mut result = new.to_rglica();
    view.add_subview(new);
    make(result.deref_mut());
    result
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}
