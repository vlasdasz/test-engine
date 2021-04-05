use crate::gm::{Rect, Color};
use crate::ui::input::Touch;
use std::rc::{Weak, Rc};
use std::any::Any;
use tools::refs::{DynWeak, Shared, MutWeak, make_shared};
use tools::weak_self::HasWeakSelf;

pub enum ViewType {
    Plain,
    Image
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait View: AsAny +  {

    fn color(&self) -> &Color;
    fn set_color(&mut self, color: Color);

    fn touch_enabled(&self) -> bool;
    fn enable_touch(&mut self);

    fn frame(&self) -> &Rect;
    fn set_frame(&mut self, frame: Rect);

    fn absolute_frame(&self) -> &Rect;
    fn calculate_absolute_frame(&mut self);

    fn superview(&self) -> DynWeak<dyn View>;
    fn set_superview(&mut self, superview: DynWeak<dyn View>);

    fn subviews(&self) -> &[Shared<dyn View>];

    fn add_subview(&mut self, view: Shared<dyn View>);
    fn remove_all_subviews(&mut self);

    fn check_touch(&self, touch: &mut Touch);

    fn setup(&mut self) { }
}

pub struct ViewBase {
    _color: Color,
    _touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    _superview: DynWeak<dyn View>,
    _subviews: Vec<Shared<dyn View>>,

    _weak: MutWeak<ViewBase>
}

impl ViewBase {

    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn make_subview(&mut self, make: fn (&mut ViewBase) -> ()) {
        let view = ViewBase::new_shared();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    pub fn on_touch(&self, touch: &Touch) {
        log!(touch);
    }

}

impl View for ViewBase {

    fn color (&self) -> &Color { &self._color }
    fn set_color(&mut self, color: Color) { self._color = color }

    fn touch_enabled (&self) ->  bool  {  self._touch_enabled  }
    fn enable_touch(&mut self) { self._touch_enabled = true }

    fn frame(&self) -> &Rect { &self._frame }

    fn set_frame(&mut self, frame: Rect)  {
        self._frame = frame
    }

    fn absolute_frame(&self) -> &Rect  { &self._absolute_frame }

    fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;

        if self._superview.is_none() {
            return;
        }

        if let Some(superview) = self._superview.as_ref().unwrap().upgrade() {
            self._absolute_frame.origin += superview.try_borrow().unwrap().absolute_frame().origin;
        };
    }

    fn superview(&self) -> DynWeak<dyn View> {
        self._superview.clone()
    }

    fn add_subview(&mut self, view: Shared<dyn View>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref.set_superview(Some(self._weak.clone()));
            mut_ref.setup();
        }
        self._subviews.push(view)
    }

    fn remove_all_subviews(&mut self) {
        self._subviews.clear()
    }

    fn set_superview(&mut self, superview: DynWeak<dyn View>) {
        self._superview = superview
    }

    fn subviews(&self) -> &[Shared<dyn View>] {
        &self._subviews
    }

    fn check_touch(&self, touch: &mut Touch) {
        if self._touch_enabled && self._absolute_frame.contains(&touch.position)  {
            touch.position -= self._absolute_frame.origin;
            self.on_touch(touch);
        }
        for view in self.subviews() {
            let borrowed = view.try_borrow().unwrap();
            borrowed.check_touch(touch);
        }
    }

}

impl AsAny for ViewBase {
    fn as_any(&self) -> &dyn Any { self }
}

impl HasWeakSelf for ViewBase {

    fn new() -> ViewBase {
        ViewBase {
            _color: Color::DEFAULT,
            _touch_enabled: false,
            _frame: Rect::new(),
            _absolute_frame: Rect::new(),
            _superview: None,
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
