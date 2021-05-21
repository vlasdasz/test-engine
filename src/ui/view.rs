use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use std::any::Any;
use std::ops::Deref;
use std::rc::Weak;
use tools::refs::{DynWeak, MutWeak, Shared};
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, New};

pub enum ViewType {
    Plain,
    Image,
}

pub trait View: AsAny {
    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
    fn setup(&mut self) {}
}

pub struct ViewBase {
    pub color: Color,
    pub touch_enabled: bool,

    _frame: Rect,
    _absolute_frame: Rect,

    _superview: DynWeak<dyn View>,
    _subviews: Vec<Shared<dyn View>>,

    _weak: MutWeak<ViewBase>,
}

impl ViewBase {
    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn make_subview(&mut self, make: fn(&mut ViewBase) -> ()) {
        let view = ViewBase::new_shared();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    pub fn on_touch(&self, touch: &Touch) {
        log!(touch);
    }

    fn touch_enabled(&self) -> bool {
        self.touch_enabled
    }

    pub fn enable_touch(&mut self) {
        self.touch_enabled = true
    }

    pub fn set_frame(&mut self, frame: Rect) {
        self._frame = frame
    }

    pub fn absolute_frame(&self) -> &Rect {
        &self._absolute_frame
    }

    pub fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;

        if self._superview.is_none() {
            return;
        }

        if let Some(superview) = self._superview.as_ref().unwrap().upgrade() {
            if let Ok(superview) = superview.try_borrow() {
                self._absolute_frame.origin += superview.view().absolute_frame().origin;
            } else {
                dbg!("fail");
            }
        };
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

    pub fn add_subview(&mut self, view: Shared<dyn View>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref.view_mut().set_superview(Some(self._weak.clone()));
            mut_ref.view_mut().setup();
        }
        self._subviews.push(view)
    }

    pub fn remove_all_subviews(&mut self) {
        self._subviews.clear()
    }

    pub fn check_touch(&self, touch: &mut Touch) {
        if self.touch_enabled && self._absolute_frame.contains(&touch.position) {
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

impl AsAny for ViewBase {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for ViewBase {
    fn new() -> ViewBase {
        ViewBase {
            color: Color::DEFAULT,
            touch_enabled: false,
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
