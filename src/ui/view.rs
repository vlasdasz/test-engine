use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use std::any::Any;

use std::rc::Weak;
use tools::refs::{DynWeak, MutWeak, Shared};
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, HasNew};

pub enum ViewType {
    Plain,
    Image,
}

pub trait View: AsAny {
    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
    fn setup(&mut self) {}

    fn color(&self) -> &Color {
        self.view()._get_color()
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut()._set_color(color)
    }

    fn frame(&self) -> &Rect {
        self.view()._frame()
    }

    fn set_frame(&mut self, rect: Rect) {
        self.view_mut()._set_frame(rect)
    }

    fn absolute_frame(&self) -> &Rect {
        self.view()._absolute_frame()
    }

    fn add_subview(&mut self, view: Shared<dyn View>) {
        self.view_mut()._add_subview(view)
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._remove_all_subviews()
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    #[derivative(Debug = "ignore")]
    _superview: DynWeak<dyn View>,
    #[derivative(Debug = "ignore")]
    _subviews: Vec<Shared<dyn View>>,

    _weak: MutWeak<ViewBase>,
}

impl ViewBase {
    pub(self) fn _frame(&self) -> &Rect {
        &self._frame
    }

    pub(self) fn _get_color(&self) -> &Color {
        &self._color
    }

    pub(self) fn _set_color(&mut self, color: Color) {
        self._color = color
    }

    pub fn make_subview(&mut self, make: fn(&mut ViewBase) -> ()) {
        let view = ViewBase::new_shared();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    pub fn on_touch(&self, touch: &Touch) {
        dbg!(touch);
    }

    fn touch_enabled(&self) -> bool {
        self._touch_enabled
    }

    pub fn enable_touch(&mut self) {
        self._touch_enabled = true
    }

    pub(self) fn _set_frame(&mut self, frame: Rect) {
        self._frame = frame
    }

    pub(self) fn _absolute_frame(&self) -> &Rect {
        &self._absolute_frame
    }

    fn super_frame(&self) -> Rect {
        return if let Some(superview) = &self._superview {
            if let Some(superview) = superview.upgrade() {
                if let Ok(superview) = superview.try_borrow() {
                    *superview.view().absolute_frame()
                } else {
                    Rect::DEFAULT
                }
            } else {
                Rect::DEFAULT
            }
        } else {
            Rect::DEFAULT
        };
    }

    pub fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;
        self._absolute_frame.origin += self.super_frame().origin
    }

    fn superview(&self) -> DynWeak<dyn View> {
        self._superview.clone()
    }

    fn set_superview(&mut self, superview: DynWeak<dyn View>) {
        self._superview = superview
    }

    pub fn subviews(&self) -> &[Shared<dyn View>] {
        &self._subviews
    }

    pub(self) fn _add_subview(&mut self, view: Shared<dyn View>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref.view_mut().set_superview(Some(self._weak.clone()));
            mut_ref.view_mut().setup();
        }
        self._subviews.push(view)
    }

    pub(self) fn _remove_all_subviews(&mut self) {
        self._subviews.clear()
    }

    pub fn check_touch(&self, touch: &mut Touch) {
        if self._touch_enabled && self._absolute_frame.contains(&touch.position) {
            touch.position -= self._absolute_frame.origin;
            self.on_touch(touch);
        }
        for view in self.subviews() {
            let borrowed = view.try_borrow().unwrap();
            borrowed.view().check_touch(touch);
        }
    }
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}

impl From<Rect> for ViewBase {
    fn from(rect: Rect) -> Self {
        let mut new = ViewBase::new();
        new._frame = rect;
        new
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
            _superview: None,
            _subviews: vec![],
            _weak: Weak::new(),
        }
    }
}

impl HasWeakSelf for ViewBase {
    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }

    fn set_weak(&mut self, weak: MutWeak<Self>) {
        self._weak = weak
    }
}
