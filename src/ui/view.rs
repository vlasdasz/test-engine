use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use std::any::Any;

use std::fmt::Debug;
use tools::refs::make_box;
use tools::{AsAny, HasNew};

pub enum ViewType {
    Plain,
    Image,
}

pub trait View: AsAny + Debug + HasNew {
    fn setup(&mut self) {}

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;

    fn ptr(&self) -> *const dyn View;

    fn color(&self) -> &Color {
        &self.view()._color
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut()._color = color
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

    fn add_subview(&mut self, mut view: Box<dyn View>) {
        view.setup();
        self.view_mut()._subviews.push(view);
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._subviews.clear()
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view()._subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut()._subviews
    }

    fn calculate_absolute_frame(&mut self, super_frame: &Rect) {
        let view = self.view_mut();
        view._absolute_frame = view._frame;
        view._absolute_frame.origin += super_frame.origin;
        let frame = view._absolute_frame;
        self.layout(super_frame);
        for view in self.subviews_mut() {
            view.calculate_absolute_frame(&frame);
        }
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
        let view = self.view();
        if view._touch_enabled && view._absolute_frame.contains(&touch.position) {
            touch.position -= view._absolute_frame.origin;
            view.on_touch(touch);
        }
        for view in self.subviews() {
            view.check_touch(touch);
        }
    }

    fn make_subview(&mut self, make: fn(&mut ViewBase) -> ()) {
        let mut view = ViewBase::new();
        make(&mut view);
        self.add_subview(make_box(view));
    }

    fn update(&mut self) {}

    fn layout(&mut self, _super_frame: &Rect) {}
}

#[derive(Debug)]
pub struct ViewBase {
    _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    _subviews: Vec<Box<dyn View>>,
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
            _subviews: vec![],
        }
    }
}
