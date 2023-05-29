use std::cell::RefCell;

use derivative::Derivative;
use gl_image::Image;
use gm::{
    flat::{Point, Rect},
    Color,
};
use refs::{Own, Weak};
use rtools::{data_manager::Handle, Unwrap};
use ui_proc::view;
use vents::Event;

use crate as ui;
use crate::{layout::Placer, NavigationView, PathData, Touch, UIAnimation, View};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) content_offset: Point,

    #[derivative(Debug = "ignore")]
    pub(crate) superview: Weak<dyn View>,

    #[derivative(Debug = "ignore")]
    pub(crate) subviews: Vec<Own<dyn View>>,

    pub(crate) touch_id: u64,

    pub(crate) image: Handle<Image>,

    pub(crate) is_selected: bool,
    pub(crate) is_deleted:  bool,

    #[derivative(Debug = "ignore")]
    pub(crate) navigation_view: Weak<NavigationView>,

    pub animations: Vec<UIAnimation>,

    pub label: String,

    #[derivative(Debug = "ignore")]
    pub place:          Unwrap<Placer>,
    pub paths:          Vec<PathData>,
    #[derivative(Debug = "ignore")]
    pub on_touch:       Event<Touch>,
    #[derivative(Debug = "ignore")]
    pub on_touch_began: Event<Touch>,
}

#[view]
pub struct Container {}
