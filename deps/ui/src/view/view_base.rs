#![allow(clippy::struct_excessive_bools)]

use educe::Educe;
use gm::{Color, flat::Rect};
use refs::{Own, Weak};
use vents::{Event, OnceEvent};

use crate::{NavigationView, Style, Touch, UIAnimation, View, WeakView, layout::Placer};

#[derive(Educe)]
#[educe(Default, Debug)]
pub struct ViewBase {
    #[educe(Debug(ignore))]
    pub(crate) color: Color,

    #[educe(Debug(ignore))]
    pub(crate) end_gradient_color: Color,

    #[educe(Debug(ignore))]
    pub(crate) corner_radius: f32,
    #[educe(Debug(ignore))]
    pub(crate) border_color:  Color,

    pub(crate) content_offset: f32,

    pub(crate) is_hidden: bool,

    #[educe(Default = crate::UIManager::ROOT_VIEW_Z_OFFSET)]
    pub(crate) z_position: f32,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    #[educe(Debug(ignore))]
    pub(crate) superview: WeakView,

    #[educe(Debug(ignore))]
    pub(crate) subviews: Vec<Own<dyn View>>,

    #[educe(Debug(ignore))]
    pub(crate) touch_id: u64,

    #[educe(Debug(ignore))]
    pub(crate) is_selected: bool,

    #[educe(Debug(ignore))]
    pub(crate) navigation_view: Weak<NavigationView>,

    #[educe(Debug(ignore))]
    pub(crate) animations: Vec<UIAnimation>,

    pub view_label: String,

    #[educe(Debug(ignore))]
    #[educe(Default = Placer::empty())]
    pub(crate) placer: Placer,

    #[educe(Debug(ignore))]
    pub touch: ViewTouchCallbacks,

    #[educe(Debug(ignore))]
    pub(crate) dont_hide_off_screen: bool,

    #[educe(Debug(ignore))]
    pub(crate) trigger_pos_changed:  bool,
    #[educe(Debug(ignore))]
    pub(crate) trigger_size_changed: bool,

    #[educe(Debug(ignore))]
    pub(crate) position_changed: Event,
    #[educe(Debug(ignore))]
    pub(crate) size_changed:     Event,

    #[educe(Debug(ignore))]
    pub(crate) after_setup: OnceEvent,

    pub(crate) ignore_global_style: bool,

    pub styles: Vec<Style>,

    pub tag: usize,
}

#[derive(Default)]
pub struct ViewTouchCallbacks {
    pub all:       Event<Touch>,
    pub began:     Event<Touch>,
    pub moved:     Event<Touch>,
    pub up_inside: Event<Touch>,
}
