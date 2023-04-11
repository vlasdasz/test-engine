use std::cell::RefCell;

use gl_image::Image;
use gm::{
    flat::{Rect, Size},
    Color,
};
use refs::{Own, Weak};
use rtools::{data_manager::Handle, Unwrap};
use smart_default::SmartDefault;
use ui_proc::view;
use vents::Event;

use crate as ui;
use crate::{layout::Placer, NavigationView, PathData, Touch, UIAnimation, View, ViewCallbacks};

#[derive(SmartDefault)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) superview: Weak<dyn View>,
    pub(crate) subviews:  Vec<Own<dyn View>>,

    pub(crate) touch_id: u64,

    pub(crate) image: Handle<Image>,

    pub(crate) is_selected: bool,
    pub(crate) is_deleted:  bool,

    pub(crate) navigation_view: Weak<NavigationView>,

    pub animations: Vec<UIAnimation>,

    pub label: String,

    pub place:          Unwrap<Placer>,
    pub paths:          Vec<PathData>,
    pub on_touch:       Event<Touch>,
    pub on_touch_began: Event<Touch>,
}

#[view]
pub struct Container {}

impl ViewCallbacks for Container {
    fn expected_size() -> Size
    where Self: Sized {
        (100, 100).into()
    }
}
