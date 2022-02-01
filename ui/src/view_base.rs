use std::ops::DerefMut;

use gm::{Color, Rect};
use rtools::{Rglica, ToRglica};

use crate::{basic::Placer, View};

#[derive(Default)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) touch_enabled: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) superview: Rglica<dyn View>,
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    pub(crate) placer: Placer,
}

impl ViewBase {
    pub fn dummy() -> Box<Self> {
        let mut dummy = Self::default();
        dummy.set_color(Color::random());
        dummy.frame_mut().size = (10, 10).into();
        Box::new(dummy)
    }
}

pub fn init_view_on<T: 'static + View>(parent: &mut dyn View) -> Rglica<T> {
    let view = T::boxed();
    let result = view.to_rglica();
    parent.add_subview(view);
    result
}

pub fn init_view_with_frame<T: 'static + View>(frame: Rect, parent: &mut dyn View) -> Rglica<T> {
    let mut view: Rglica<T> = init_view_on(parent);
    view.set_frame(frame);
    view
}

pub fn make_view_on<T: 'static + View>(
    parent: &mut dyn View,
    make: impl FnOnce(&mut T),
) -> Rglica<T> {
    let mut view: Rglica<T> = init_view_on(parent);
    make(view.deref_mut());
    view
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}
