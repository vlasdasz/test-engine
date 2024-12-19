use educe::Educe;
use gm::ToF32;
use refs::Weak;

use crate::{
    WeakView,
    layout::{Anchor, Tiling},
};

#[derive(Educe)]
#[educe(Debug)]
pub(crate) struct LayoutRule {
    pub(crate) side:   Anchor,
    pub(crate) tiling: Option<Tiling>,
    pub(crate) offset: f32,

    #[educe(Debug(ignore))]
    pub(crate) anchor_view:  WeakView,
    #[educe(Debug(ignore))]
    pub(crate) anchor_view2: WeakView,

    pub(crate) relative: bool,
    pub(crate) between:  bool,
}

impl LayoutRule {
    pub fn tiling(tiling: Tiling, offset: impl ToF32) -> Self {
        Self {
            side:         Anchor::Top,
            tiling:       tiling.into(),
            offset:       offset.to_f32(),
            anchor_view:  Weak::default(),
            anchor_view2: Weak::default(),
            relative:     false,
            between:      false,
        }
    }

    pub fn make(side: Anchor, offset: impl ToF32) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.to_f32(),
            anchor_view: Weak::default(),
            anchor_view2: Weak::default(),
            relative: false,
            between: false,
        }
    }

    pub fn anchor(side: Anchor, offset: impl ToF32, anchor_view: WeakView) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.to_f32(),
            anchor_view,
            anchor_view2: Weak::default(),
            relative: false,
            between: false,
        }
    }

    pub fn relative(side: Anchor, ratio: impl ToF32, anchor_view: WeakView) -> Self {
        Self {
            side,
            tiling: None,
            offset: ratio.to_f32(),
            anchor_view,
            anchor_view2: Weak::default(),
            relative: true,
            between: false,
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
        }
    }
}

impl LayoutRule {
    pub(crate) fn is_center(&self) -> bool {
        matches!(self.side, Anchor::Center | Anchor::CenterX | Anchor::CenterY)
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
