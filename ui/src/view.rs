use crate::input::Touch;
use gl_image::Image;
use gm::{Color, Rect};
use proc_macro::AsAny;
use proc_macro::New;
use std::cell::RefCell;
use tools::refs::Shared;
use tools::{AsAny, Event, New, Rglica};

pub trait View: AsAny + New {
    fn setup(&mut self, _this: Shared<dyn View>) {}

    fn update(&mut self) {}

    fn layout(&mut self, _super_frame: &Rect) {}

    fn color(&self) -> &Color {
        &self.view()._color
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut()._color = color
    }

    fn super_frame(&self) -> &Rect {
        if self.view()._superview.is_ok() {
            return self.view()._superview.frame();
        }
        self.frame()
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

    fn absolute_frame(&self) -> &Rect {
        &self.view()._absolute_frame
    }

    fn add_subview(&mut self, view: Shared<dyn View>) {
        view.borrow_mut().setup(view.clone());
        self.view_mut()._subviews.push(view);
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._subviews.clear()
    }

    fn subviews(&self) -> &[Shared<dyn View>] {
        &self.view()._subviews
    }

    fn subviews_mut(&mut self) -> &mut [Shared<dyn View>] {
        &mut self.view_mut()._subviews
    }

    fn calculate_absolute_frame(&mut self, _super_frame: &Rect) {
        let view = self.view_mut();
        view._absolute_frame = view._frame;
        view._absolute_frame.origin += view.super_frame().origin;
        let frame = view._absolute_frame;
        self.layout(&Rect::DEFAULT);
        for view in self.subviews_mut() {
            view.try_borrow_mut()
                .unwrap()
                .calculate_absolute_frame(&frame);
        }
    }

    fn enable_touch(&mut self) {
        self.view_mut()._touch_enabled = true
    }

    fn touch_enabled(&self) -> bool {
        self.view()._touch_enabled
    }

    fn handle_touch(&self, touch: &Touch) {
        self.view()._on_touch.trigger(touch);
    }

    fn touch_id(&self) -> u64 {
        self.view()._touch_id.borrow().clone()
    }

    fn set_touch_id(&self, id: u64) {
        *self.view()._touch_id.borrow_mut() = id;
    }

    fn from_rect(rect: Rect) -> Self
    where
        Self: Sized,
    {
        let mut new = Self::new();
        new.set_frame(rect);
        new
    }

    fn on_touch(&mut self) -> &mut Event<Touch> {
        &mut self.view_mut()._on_touch
    }

    fn check_touch(&self, touch: &mut Touch) -> bool {
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

        for view in self.subviews() {
            if view.borrow().check_touch(touch) {
                return true;
            }
        }

        false
    }

    fn image(&self) -> Option<Image> {
        None
    }

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}

#[derive(AsAny, New)]
pub struct ViewBase {
    _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    _superview: Rglica<dyn View>,

    _subviews: Vec<Shared<dyn View>>,

    _on_touch: Event<Touch>,
    _touch_id: RefCell<u64>,
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}
