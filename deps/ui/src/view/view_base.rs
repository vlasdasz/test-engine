use std::cell::RefCell;

use derivative::Derivative;
use gl_image::Image;
use gm::{flat::Rect, Color};
use rtools::{data_manager::Handle, Event, IntoF32, Rglica, ToRglica};

use crate::{
    basic::Placer,
    complex::PathData,
    view::{ViewData, ViewFrame},
    Touch, UIDrawer, View,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) touch_enabled: RefCell<bool>,
    pub(crate) on_touch:      Event<Touch>,

    pub(crate) is_hidden: bool,

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

    pub(crate) image: Handle<Image>,

    pub(crate) paths: Vec<PathData>,

    pub(crate) drawer: Rglica<dyn UIDrawer>,
}

impl ViewBase {
    pub fn dummy() -> Box<Self> {
        let mut dummy = Self::default();
        dummy.set_frame((5, 5)).set_color(Color::random());
        Box::new(dummy)
    }
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }

    fn rglica(&self) -> Rglica<dyn View> {
        (self as &dyn View).to_rglica()
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
