use std::cell::RefCell;

use derivative::Derivative;
use gl_image::Image;
use gm::{flat::Rect, Color};
use rtools::{data_manager::Handle, Event, Rglica, ToRglica};

use crate::{
    basic::RootView,
    complex::PathData,
    layout::{NewPlacer, Placer, Tiling},
    Touch, UIDrawer, View,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,
    pub(crate) on_touch:      Event<Touch>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    #[derivative(Debug = "ignore")]
    pub(crate) absolute_frame: Rect,

    #[derivative(Debug = "ignore")]
    pub(crate) root_view: Rglica<RootView>,

    #[derivative(Debug = "ignore")]
    pub(crate) superview: Rglica<dyn View>,
    #[derivative(Debug = "ignore")]
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    #[derivative(Debug = "ignore")]
    pub(crate) placer: Placer,

    #[derivative(Debug = "ignore")]
    pub(crate) new_placer: NewPlacer,

    #[derivative(Debug = "ignore")]
    pub(crate) tiling: Option<Tiling>,

    pub(crate) image: Handle<Image>,

    pub(crate) paths: Vec<PathData>,

    pub(crate) drawer: Rglica<dyn UIDrawer>,
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
