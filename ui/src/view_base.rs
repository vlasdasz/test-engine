use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use gm::{Color, Rect};
use rtools::{Rglica, ToRglica};

use crate::{basic::Placer, View};

#[derive(Default)]
pub struct ViewBase {
    pub color: Color,

    pub(crate) touch_enabled: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) superview: Rglica<dyn View>,
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: RefCell<u64>,

    pub(crate) placer: Placer,
}

impl ViewBase {
    pub fn make_view<T: 'static + View>(&mut self) -> Rglica<T> {
        let view = T::boxed();
        let rglica = view.to_rglica();
        self.add_subview(view);
        rglica
    }

    pub fn make_view_with<T: 'static + View>(&mut self, frame: Rect) -> Rglica<T> {
        let view = T::with_frame(frame);
        let rglica = view.to_rglica();
        self.add_subview(view);
        rglica
    }

    pub fn dummy() -> Box<Self> {
        let mut dummy = Self::default();
        dummy.set_color(Color::random());
        dummy.frame_mut().size = (10, 10).into();
        Box::new(dummy)
    }
}

pub fn init_view_on<T: 'static + View>(view: &mut dyn View) -> Rglica<T> {
    view.view_mut().make_view()
}

pub fn init_view_with_frame<T: 'static + View>(frame: Rect, view: &mut dyn View) -> Rglica<T> {
    view.view_mut().make_view_with(frame)
}

pub fn make_view_on<T: 'static + View>(
    view: &mut dyn View,
    make: impl FnOnce(&mut T),
) -> Rglica<T> {
    let new = T::boxed();
    let mut result = new.to_rglica();
    view.add_subview(new);
    make(result.deref_mut());
    result
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}

impl Deref for dyn View {
    type Target = ViewBase;
    fn deref(&self) -> &ViewBase {
        self.view()
    }
}

impl DerefMut for dyn View {
    fn deref_mut(&mut self) -> &mut ViewBase {
        self.view_mut()
    }
}
