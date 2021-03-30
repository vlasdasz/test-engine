use crate::gm::{Rect, Color};
use crate::ui::input::Touch;
pub use crate::utils::{Shared, make_shared, MutWeak, HasWeakSelf};
use std::rc::{Weak, Rc};

pub trait View: HasWeakSelf {

    fn color(&self) -> &Color;
    fn touch_enabled(&self) -> bool;

    fn absolute_frame(&self) -> &Rect;
    fn calculate_absolute_frame(&mut self);

    fn superview(&self) -> MutWeak<dyn View>;
    fn subviews(&self) -> &[Shared<dyn View>];
}

pub struct ViewBase {
    pub _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _super_frame: Rect,
    _absolute_frame: Rect,
    _needs_layout: bool,

    _superview: MutWeak<ViewBase>,
    _subviews: Vec<Shared<dyn View>>,

    _weak: MutWeak<ViewBase>
}

impl ViewBase {

    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn absolute_frame(&self) -> &Rect {
        &self._absolute_frame
    }

    pub fn set_frame(&mut self, frame: Rect)  {
        self._frame = frame
    }

    pub fn add_subview(&mut self, view: Shared<ViewBase>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref._superview = self._weak.clone();
        }
        self._subviews.push(view)
    }

    pub fn make_subview(&mut self, make: fn (&mut ViewBase) -> ()) {
        let view = ViewBase::new_shared();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    pub fn check_touch(&self, touch: &mut Touch) {
        // if self.touch_enabled && self._absolute_frame.contains(&touch.position)  {
        //     touch.position -= self._absolute_frame.origin;
        //     self.on_touch(touch);
        // }
        // for view in self.subviews() {
        //     let borrowed = view.try_borrow().unwrap();
        //     borrowed.check_touch(touch);
        // }
    }

    pub fn on_touch(&self, touch: &Touch) {
        log!(touch);
    }

}

impl View for ViewBase {

    fn color         (&self) -> &Color { &self._color          }
    fn touch_enabled (&self) ->  bool  {  self._touch_enabled  }
    fn absolute_frame(&self) -> &Rect  { &self._absolute_frame }

    fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;

        if let Some(superview) = self._superview.upgrade() {
            self._absolute_frame.origin += superview.try_borrow().unwrap()._absolute_frame.origin;
        };
    }

    fn superview(&self) -> MutWeak<dyn View> {
        self._weak.clone()
    }

    fn subviews(&self) -> &[Shared<dyn View>] {
        &self._subviews
    }

}

impl HasWeakSelf for ViewBase {

    fn new() -> ViewBase {
        ViewBase {
            _color: Color::DEFAULT,
            _touch_enabled: false,
            _frame: Rect::new(),
            _super_frame: Rect::new(),
            _absolute_frame: Rect::new(),
            _needs_layout: true,
            _superview: Weak::new(),
            _subviews: vec!(),
            _weak: Weak::new()
        }
    }

    fn new_shared() -> Shared<Self> {
        let result = make_shared(ViewBase::new());
        result.try_borrow_mut().unwrap()._weak = Rc::downgrade(&result);
        result
    }

    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }
}
