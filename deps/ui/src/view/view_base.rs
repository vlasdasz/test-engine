use std::cell::RefCell;

use gl_image::Image;
use gm::{flat::Rect, Color};
use rtools::{data_manager::Handle, Event, Rglica, ToRglica};

use crate::{basic::RootView, complex::PathData, layout::Placer, view, Touch, UIDrawer, View};

#[derive(Default)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,
    pub(crate) on_touch:      Event<Touch>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) root_view: Rglica<RootView>,

    pub(crate) superview: Rglica<dyn View>,
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    pub(crate) placer: Placer,

    pub(crate) image: Handle<Image>,

    pub(crate) paths: Vec<PathData>,

    pub(crate) drawer: Rglica<dyn UIDrawer>,
}

#[view]
#[derive(Default)]
pub struct BaseView {}
