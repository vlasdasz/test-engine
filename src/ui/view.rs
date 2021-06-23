use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use std::any::Any;

use std::rc::Weak;
use tools::refs::{DynWeak, MutWeak, Shared, make_box};
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, HasNew};
use std::fmt::Debug;
use std::ptr::null;

pub enum ViewType {
    Plain,
    Image,
}

pub trait View: AsAny + HasNew + Debug {

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;

    fn ptr(&self) -> *const dyn View;

    fn setup(&mut self) {}

    fn color(&self) -> &Color {
        &self.view()._color
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut()._color = color
    }

    fn frame(&self) -> &Rect {
        &self.view()._frame
    }

    fn set_frame(&mut self, rect: Rect) {
        self.view_mut()._frame = rect
    }

    fn absolute_frame(&self) -> &Rect {
        &self.view()._absolute_frame
    }

    fn add_subview(&mut self, mut view: Box<dyn View>) {
        view.view_mut()._superview = self.ptr();
        self.view_mut()._subviews.push(view)
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._subviews.clear()
    }

    fn subviews(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut()._subviews
    }

    fn superview(&self) -> Option<&dyn View> {
        if self.view()._superview.is_null() {
            return None;
        }
        Some(unsafe { &*self.view()._superview })
    }

    fn calculate_absolute_frame(&mut self) {
        let view = self.view_mut();
        view._absolute_frame = view._frame;
        view._absolute_frame.origin += view.super_frame().origin
    }

    fn enable_touch(&mut self) {
        self.view_mut()._touch_enabled = true
    }

    fn on_touch(&self, touch: &Touch) {
        dbg!(touch);
    }

    fn from_rect(rect: Rect) -> Self
    where
        Self: Sized,
    {
        let mut new = Self::new();
        new.set_frame(rect);
        new
    }

    fn check_touch(&self, touch: &mut Touch) {
        // let view = self.view();
        // if view._touch_enabled && view._absolute_frame.contains(&touch.position) {
        //     touch.position -= view._absolute_frame.origin;
        //     view.on_touch(touch);
        // }
        // for view in view.subviews() {
        //     let borrowed = view.try_borrow().unwrap();
        //     borrowed.view().check_touch(touch);
        // }
    }

    fn make_subview(&mut self, make: fn(&mut ViewBase) -> ()) {
        let mut view = ViewBase::new();
        make(&mut view);
        self.add_subview(make_box(view));
    }

    fn super_frame(&self) -> &Rect {
        if let Some(superview) = self.superview() {
            return superview.frame();
        }
        return &Rect::DEFAULT;
    }
}

#[derive(Debug)]
pub struct ViewBase {
    _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    _superview: *const dyn View,
    _subviews: Vec<Box<dyn View>>
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }

    fn ptr(&self) -> *const dyn View {
        self as *const dyn View
    }
}

impl AsAny for ViewBase {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasNew for ViewBase {
    fn new() -> ViewBase {
        ViewBase {
            _color: Color::DEFAULT,
            _touch_enabled: false,
            _frame: Rect::new(),
            _absolute_frame: Rect::new(),
            _superview: null::<Self>(),
            _subviews: vec![],
        }
    }
}
