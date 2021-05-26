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

pub trait View: AsAny + HasNew {
    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
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

    fn add_subview(&mut self, view: Shared<dyn View>) {
        {
            let mut mut_ref = view.try_borrow_mut().unwrap();
            mut_ref.view_mut()._superview = Some(self.view()._weak.clone());
            mut_ref.view_mut().setup();
        }
        self.view_mut()._subviews.push(view)
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut()._subviews.clear()
    }

    fn subviews(&self) -> &[Shared<dyn View>] {
        &self.view()._subviews
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
        let view = self.view();
        if view._touch_enabled && view._absolute_frame.contains(&touch.position) {
            touch.position -= view._absolute_frame.origin;
            view.on_touch(touch);
        }
        for view in view.subviews() {
            let borrowed = view.try_borrow().unwrap();
            borrowed.view().check_touch(touch);
        }
    }

    fn make_subview(&mut self, make: fn(&mut ViewBase) -> ()) {
        let view = ViewBase::new_shared();
        make(&mut view.try_borrow_mut().unwrap());
        self.add_subview(view);
    }

    fn super_frame(&self) -> Rect {
        if let Some(superview) = &self.view()._superview {
            if let Some(superview) = superview.upgrade() {
                if let Ok(superview) = superview.try_borrow() {
                    *superview.absolute_frame()
                } else {
                    //panic!("KOKOKO!!");
                    Rect::DEFAULT
                }
            } else {
                Rect::DEFAULT
            }
        } else {
            return Rect::DEFAULT;
        }
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
