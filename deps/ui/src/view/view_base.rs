use std::cell::RefCell;

use derivative::Derivative;
use gl_image::Image;
use gm::{
    flat::{Point, Rect},
    Color,
};
use refs::{Own, Weak};
use rtools::{data_manager::Handle, Unwrap};
use vents::Event;

use crate::{layout::Placer, NavigationView, PathData, Touch, UIAnimation, View};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub content_offset: Point,

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

    pub priority: usize,

    pub dont_hide: bool,

    #[derivative(Debug = "ignore")]
    after_setup: RefCell<Vec<Box<dyn FnOnce()>>>,

    #[derivative(Debug = "ignore")]
    before_setup: RefCell<Vec<Box<dyn FnOnce()>>>,
}

impl ViewBase {
    pub fn after_setup(&self, action: impl FnOnce() + 'static) {
        self.after_setup.borrow_mut().push(Box::new(action))
    }

    pub fn __trigger_after_setup(&self) {
        for action in self.after_setup.borrow_mut().drain(..) {
            action()
        }
    }

    pub fn before_setup(&self, action: impl FnOnce() + 'static) {
        self.before_setup.borrow_mut().push(Box::new(action))
    }

    pub fn __trigger_before_setup(&self) {
        for action in self.before_setup.borrow_mut().drain(..) {
            action()
        }
    }
}
