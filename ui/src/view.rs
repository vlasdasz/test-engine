use std::{cell::RefCell, ops::DerefMut};

use gl_image::Image;
use gm::{Color, Rect};
use proc_macro::{AsAny, Boxed};
use tools::{new, rglica::ToRglica, AsAny, Boxed, Event, New, Rglica};

use crate::{basic::Placer, input::Touch};

pub trait View: AsAny + Boxed {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn layout(&mut self) {}

    fn color(&self) -> &Color { &self.view()._color }

    fn set_color(&mut self, color: Color) { self.view_mut()._color = color }

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

    fn frame(&self) -> &Rect { &self.view()._frame }

    fn frame_mut(&mut self) -> &mut Rect { &mut self.view_mut()._frame }

    fn set_frame(&mut self, rect: Rect) { self.view_mut()._frame = rect }

    fn absolute_frame(&self) -> &Rect { &self.view()._absolute_frame }

    fn add_subview(&mut self, mut view: Box<dyn View>) {
        view.view_mut()._superview = Rglica::from_ref(self.view());
        view.view_mut()._placer = Placer::make(view.deref_mut());
        view.setup();
        self.view_mut()._subviews.push(view);
    }

    fn remove_all_subviews(&mut self) { self.view_mut()._subviews.clear() }

    fn subviews(&self) -> &[Box<dyn View>] { &self.view()._subviews }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] { &mut self.view_mut()._subviews }

    fn calculate_absolute_frame(&mut self) {
        let view = self.view_mut();
        view._absolute_frame = view._frame;
        view._absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_absolute_frame();
        }
    }

    fn enable_touch(&mut self) { self.view_mut()._touch_enabled = true }

    fn touch_enabled(&self) -> bool { self.view()._touch_enabled }

    fn handle_touch(&mut self, touch: &Touch) { self.view_mut()._on_touch.trigger(*touch); }

    fn touch_id(&self) -> u64 { *self.view()._touch_id.borrow() }

    fn set_touch_id(&self, id: u64) { *self.view()._touch_id.borrow_mut() = id; }

    fn on_touch(&mut self) -> &mut Event<Touch> { &mut self.view_mut()._on_touch }

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

    fn image(&self) -> Option<Image> { None }

    fn placer(&mut self) -> &mut Placer { &mut self.view_mut()._placer }

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}

#[derive(AsAny, Boxed)]
pub struct ViewBase {
    _color:         Color,
    _touch_enabled: bool,

    _frame:          Rect,
    _absolute_frame: Rect,

    _superview: Rglica<ViewBase>,

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
}

pub fn make_view_on<T: 'static + View>(view: &mut dyn View) -> Rglica<T> {
    view.view_mut().make_view()
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase { self }

    fn view_mut(&mut self) -> &mut Self { self }
}

impl Default for ViewBase {
    fn default() -> Self {
        Self {
            _color:         Color::DEFAULT,
            _touch_enabled: false,

            _frame:          Rect::DEFAULT,
            _absolute_frame: Rect::DEFAULT,

            _superview: Rglica::new(),

            _subviews: vec![],

            _on_touch: Event::new(),
            _touch_id: new(),

            _placer: Placer::new(),
        }
    }
}
