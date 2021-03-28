use crate::gm::{Rect, Color, Point};
use crate::ui::input::Touch;
use crate::utils::{Shared, make_shared};
use std::rc::{Weak, Rc};
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub type SuperView = Weak<RefCell<View>>;

#[derive(Debug)]
pub struct View {
    pub color: Color,

    _frame: Rect,
    _super_frame: Rect,
    _absolute_frame: Rect,
    _needs_layout: bool,

    _superview: SuperView,
    _subviews: Vec<Shared<View>>,

    _weak: SuperView
}

impl View {
    pub fn new() -> Shared<View> {
        let result = make_shared(
            View {
                _frame: Rect::new(),
                _super_frame: Rect::new(),
                _absolute_frame: Rect::new(),
                color: Color::DEFAULT,
                _needs_layout: true,
                _superview: Weak::new(),
                _subviews: vec!(),
                _weak: Weak::new()
            }
        );

        result.try_borrow_mut().unwrap()._weak = Rc::downgrade(&result);

        result
    }

    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn absolute_frame(&self) -> &Rect {
        &self._absolute_frame
    }

    pub fn set_frame(&mut self, frame: Rect)  {
        self._frame = frame
    }

    pub fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;

        if let Some(superview) = self._superview.upgrade() {
            self._absolute_frame.origin += superview.try_borrow().unwrap()._absolute_frame.origin;
        };
    }

    pub fn add_subview(&mut self, view: Shared<View>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref._superview = self._weak.clone();
        }
        self._subviews.push(view)
    }

    pub fn make_subview(&mut self, make: fn (&mut View) -> ()) {
        let view = View::new();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    pub fn subviews(&self) -> &[Shared<View>] {
        &self._subviews
    }

}

impl View {

    pub fn contains_global_point(&self, point: &Point) -> bool {
        self._absolute_frame.contains(point)
    }

    pub fn on_touch(&mut self, touch: Touch) {
        log!(touch)
    }
}