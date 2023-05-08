use refs::Weak;
use rtools::IntoF32;

use crate::{
    layout::{Anchor, Tiling},
    View,
};

pub(crate) struct LayoutRule {
    pub(crate) side:   Anchor,
    pub(crate) tiling: Option<Tiling>,
    pub(crate) offset: f32,

    pub(crate) anchor_view:  Weak<dyn View>,
    pub(crate) anchor_view2: Weak<dyn View>,

    pub(crate) relative: bool,
    pub(crate) between:  bool,
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
        }
    }

    pub fn anchor(side: Anchor, offset: impl IntoF32, anchor_view: Weak<dyn View>) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.into_f32(),
            anchor_view,
            anchor_view2: Default::default(),
            relative: false,
            between: false,
        }
    }

    pub fn relative(side: Anchor, ratio: impl IntoF32, anchor_view: Weak<dyn View>) -> Self {
        Self {
            side,
            tiling: None,
            offset: ratio.into_f32(),
            anchor_view,
            anchor_view2: Default::default(),
            relative: true,
            between: false,
        }
    }

    pub fn between(view_a: Weak<dyn View>, view_b: Weak<dyn View>, side: Anchor) -> Self {
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
