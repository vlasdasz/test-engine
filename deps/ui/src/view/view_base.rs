#![allow(clippy::struct_excessive_bools)]

use derivative::Derivative;
use gm::{flat::Rect, Color};
use refs::{Own, Weak};
use vents::{Event, OnceEvent};

use crate::{layout::Placer, NavigationView, Touch, UIAnimation, View, WeakView};

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct ViewBase {
    #[derivative(Debug = "ignore")]
    pub(crate) color: Color,

    #[derivative(Debug = "ignore")]
    pub(crate) corner_radius: f32,
    #[derivative(Debug = "ignore")]
    pub(crate) border_color:  Color,

    pub(crate) content_offset: f32,

    pub(crate) is_hidden: bool,

    #[derivative(Default(value = "crate::UIManager::ROOT_VIEW_Z_OFFSET"))]
    pub(crate) z_position: f32,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    #[derivative(Debug = "ignore")]
    pub(crate) superview: WeakView,

    #[derivative(Debug = "ignore")]
    pub(crate) subviews: Vec<Own<dyn View>>,

    #[derivative(Debug = "ignore")]
    pub(crate) touch_id: u64,

    #[derivative(Debug = "ignore")]
    pub(crate) is_selected: bool,

    #[derivative(Debug = "ignore")]
    pub(crate) navigation_view: Weak<NavigationView>,

    #[derivative(Debug = "ignore")]
    pub(crate) animations: Vec<UIAnimation>,

    pub view_label: String,

    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "Placer::empty()"))]
    pub(crate) placer: Placer,

    #[derivative(Debug = "ignore")]
    pub touch: ViewTouchCallbacks,

    #[derivative(Debug = "ignore")]
    pub dont_hide: bool,

    #[derivative(Debug = "ignore")]
    pub(crate) loaded: Event,

    #[derivative(Debug = "ignore")]
    pub(crate) trigger_pos_changed:  bool,
    #[derivative(Debug = "ignore")]
    pub(crate) trigger_size_changed: bool,

    #[derivative(Debug = "ignore")]
    pub(crate) position_changed: Event,
    #[derivative(Debug = "ignore")]
    pub(crate) size_changed:     Event,

    #[derivative(Debug = "ignore")]
    pub(crate) after_setup: OnceEvent,
}

#[derive(Default)]
pub struct ViewTouchCallbacks {
    pub all:       Event<Touch>,
    pub began:     Event<Touch>,
    pub up_inside: Event<Touch>,
}
