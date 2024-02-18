use derivative::Derivative;
use gm::flat::Size;
use rtools::IntoF32;

use crate::{
    layout::{Anchor, Tiling},
    WeakView,
};

pub(crate) type CustomCallback = Box<dyn FnMut(WeakView, &Size)>;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct LayoutRule {
    pub(crate) side:   Anchor,
    pub(crate) tiling: Option<Tiling>,
    pub(crate) offset: f32,

    #[derivative(Debug = "ignore")]
    pub(crate) anchor_view:  WeakView,
    #[derivative(Debug = "ignore")]
    pub(crate) anchor_view2: WeakView,

    pub(crate) relative: bool,
    pub(crate) between:  bool,

    #[derivative(Debug = "ignore")]
    pub(crate) custom: Option<CustomCallback>,
}

impl LayoutRule {
    pub fn tiling(tiling: Tiling, offset: impl IntoF32) -> Self {
        Self {
            side:         Anchor::Top,
            tiling:       tiling.into(),
            offset:       offset.into_f32(),
            anchor_view:  Default::default(),
            anchor_view2: Default::default(),
            relative:     false,
            between:      false,
            custom:       None,
        }
    }

    pub fn make(side: Anchor, offset: impl IntoF32) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.into_f32(),
            anchor_view: Default::default(),
            anchor_view2: Default::default(),
            relative: false,
            between: false,
            custom: None,
        }
    }

    pub fn anchor(side: Anchor, offset: impl IntoF32, anchor_view: WeakView) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.into_f32(),
            anchor_view,
            anchor_view2: Default::default(),
            relative: false,
            between: false,
            custom: None,
        }
    }

    pub fn relative(side: Anchor, ratio: impl IntoF32, anchor_view: WeakView) -> Self {
        Self {
            side,
            tiling: None,
            offset: ratio.into_f32(),
            anchor_view,
            anchor_view2: Default::default(),
            relative: true,
            between: false,
            custom: None,
        }
    }

    pub fn between(view_a: WeakView, view_b: WeakView, side: Anchor) -> Self {
        Self {
            side,
            tiling: None,
            offset: 0.0,
            anchor_view: view_a,
            anchor_view2: view_b,
            relative: false,
            between: true,
            custom: None,
        }
    }

    pub fn custom(action: impl FnMut(WeakView, &Size) + 'static) -> Self {
        Self {
            side:         Anchor::Bot,
            tiling:       None,
            offset:       0.0,
            anchor_view:  Default::default(),
            anchor_view2: Default::default(),
            relative:     false,
            between:      false,
            custom:       Some(Box::new(action)),
        }
    }
}

impl From<Anchor> for LayoutRule {
    fn from(anchor: Anchor) -> Self {
        Self::make(anchor, 0)
    }
}

impl From<Tiling> for LayoutRule {
    fn from(tiling: Tiling) -> Self {
        Self::tiling(tiling, 0)
    }
}
