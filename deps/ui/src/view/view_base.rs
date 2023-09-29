use derivative::Derivative;
use gm::{
    flat::{Point, Rect},
    Color,
};
use refs::{Own, Weak};
use vents::Event;

use crate::{layout::Placer, NavigationView, Touch, UIAnimation, View};

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

    pub(crate) is_selected: bool,
    pub(crate) is_deleted:  bool,

    #[derivative(Debug = "ignore")]
    pub(crate) navigation_view: Weak<NavigationView>,

    pub animations: Vec<UIAnimation>,

    pub label: String,

    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "Placer::empty()"))]
    pub place: Placer,

    #[derivative(Debug = "ignore")]
    pub touch: ViewTouchCallbacks,

    pub priority: usize,

    pub dont_hide: bool,

    pub loaded: Event,
}

#[derive(Default)]
pub struct ViewTouchCallbacks {
    pub all:   Event<Touch>,
    pub began: Event<Touch>,
}
