use std::ops::DerefMut;

use derivative::Derivative;
use gm::{Color, Rect};
use rtools::{Boxed, IntoF32, Rglica, ToRglica};

use crate::{basic::Placer, complex::Alert, View};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) touch_enabled: bool,

    pub(crate) frame:          Rect,
    #[derivative(Debug = "ignore")]
    pub(crate) absolute_frame: Rect,

    #[derivative(Debug = "ignore")]
    pub(crate) superview: Rglica<dyn View>,
    #[derivative(Debug = "ignore")]
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    #[derivative(Debug = "ignore")]
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

pub fn add_view<T: 'static + View>(parent: &mut dyn View) -> Rglica<T> {
    let view = T::boxed();
    let result = view.to_rglica();
    parent.add_subview(view);
    result
}

pub fn add_view_with_frame<T: 'static + View>(parent: &mut dyn View, frame: Rect) -> Rglica<T> {
    let mut view: Rglica<T> = add_view(parent);
    view.set_frame(frame);
    view
}

pub fn make_view_on<T: 'static + View>(parent: &mut dyn View, make: impl FnOnce(&mut T)) -> Rglica<T> {
    let mut view: Rglica<T> = add_view(parent);
    make(view.deref_mut());
    view
}

pub fn alert(view: &mut dyn View, message: impl ToString) {
    let mut alert = Alert::boxed();
    alert.set_message(message);
    view.root_view().add_subview(alert);
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}

impl<W: IntoF32, H: IntoF32> From<(W, H)> for Box<dyn View> {
    fn from(data: (W, H)) -> Self {
        Box::new(ViewBase {
            frame: (data.0, data.1).into(),
            ..Default::default()
        })
    }
}

impl PartialEq for ViewBase {
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame
            && self.absolute_frame == other.absolute_frame
            && self.color == other.color
            && self.touch_enabled == other.touch_enabled
    }
}
