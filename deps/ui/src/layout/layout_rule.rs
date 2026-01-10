use educe::Educe;
use gm::ToF32;

use crate::{
    WeakView,
    layout::{Anchor, Tiling},
};

#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct LayoutRule {
    pub side:   Option<Anchor>,
    pub tiling: Option<Tiling>,
    pub offset: f32,

    #[educe(Debug(ignore))]
    pub anchor_view:  Option<WeakView>,
    #[educe(Debug(ignore))]
    pub anchor_view2: Option<WeakView>,

    pub relative: bool,
    pub between:  bool,
    pub same:     bool,
}

impl LayoutRule {
    pub fn tiling(tiling: Tiling, offset: impl ToF32) -> Self {
        Self {
            side:         None,
            tiling:       tiling.into(),
            offset:       offset.to_f32(),
            anchor_view:  None,
            anchor_view2: None,
            relative:     false,
            between:      false,
            same:         false,
        }
    }

    pub fn make(side: Anchor, offset: impl ToF32) -> Self {
        Self {
            side:         Some(side),
            tiling:       None,
            offset:       offset.to_f32(),
            anchor_view:  None,
            anchor_view2: None,
            relative:     false,
            between:      false,
            same:         false,
        }
    }

    pub fn anchor(side: Anchor, offset: impl ToF32, anchor_view: WeakView) -> Self {
        Self {
            side:         Some(side),
            tiling:       None,
            offset:       offset.to_f32(),
            anchor_view:  Some(anchor_view),
            anchor_view2: None,
            relative:     false,
            between:      false,
            same:         false,
        }
    }

    pub fn relative(side: Anchor, ratio: impl ToF32, anchor_view: WeakView) -> Self {
        Self {
            side:         Some(side),
            tiling:       None,
            offset:       ratio.to_f32(),
            anchor_view:  Some(anchor_view),
            anchor_view2: None,
            relative:     true,
            between:      false,
            same:         false,
        }
    }

    pub fn same(side: Anchor, anchor_view: WeakView) -> Self {
        Self {
            side:         Some(side),
            tiling:       None,
            offset:       0.0,
            anchor_view:  Some(anchor_view),
            anchor_view2: None,
            relative:     false,
            between:      false,
            same:         true,
        }
    }

    pub fn between(view_a: WeakView, view_b: WeakView, side: Option<Anchor>) -> Self {
        Self {
            side,
            tiling: None,
            offset: 0.0,
            anchor_view: Some(view_a),
            anchor_view2: Some(view_b),
            relative: false,
            between: true,
            same: false,
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
