use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{flat::Rect, Color};
use refs::{Own, ToWeak, Weak};
use rtools::{data_manager::Handle, Event, Unwrap};

use crate::{layout::Placer, PathData, Touch, View};

#[derive(Default)]
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

    pub place:          Unwrap<Placer>,
    pub paths:          Vec<PathData>,
    pub on_touch:       Event<Touch>,
    pub on_touch_began: Event<Touch>,
}

#[derive(Default)]
pub struct BaseView {
    view: ViewBase,
}

impl View for BaseView {
    fn init_views(&mut self) {}

    fn weak_view(&self) -> Weak<dyn View> {
        (self as &dyn View).weak()
    }
}

impl Deref for BaseView {
    type Target = ViewBase;

    fn deref(&self) -> &ViewBase {
        &self.view
    }
}

impl DerefMut for BaseView {
    fn deref_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}
