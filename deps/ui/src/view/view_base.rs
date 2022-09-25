use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{flat::Rect, Color};
use rtools::{data_manager::Handle, Rglica, ToRglica, Unwrap};

use crate::{layout::Placer, PathData, View};

#[derive(Default)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) superview: Rglica<dyn View>,
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    pub place: Unwrap<Placer>,

    pub(crate) image: Handle<Image>,

    pub paths: Vec<PathData>,

    pub(crate) is_selected: bool,

    pub(crate) is_deleted: bool,
}

#[derive(Default)]
pub struct BaseView {
    view: ViewBase,
}

impl View for BaseView {
    fn init_views(&mut self) {}

    fn rglica(&self) -> Rglica<dyn View> {
        (self as &dyn View).to_rglica()
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
