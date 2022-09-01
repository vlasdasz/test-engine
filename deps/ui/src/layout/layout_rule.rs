use rtools::{IntoF32, Rglica};

use crate::{
    layout::{Anchor, Tiling},
    View,
};

pub(crate) struct LayoutRule {
    pub(crate) side:   Anchor,
    pub(crate) tiling: Option<Tiling>,
    pub(crate) offset: f32,

    pub(crate) anchor_view: Rglica<dyn View>,
}

impl LayoutRule {
    pub fn tiling(tiling: Tiling, offset: impl IntoF32) -> Self {
        Self {
            side:        Anchor::Top,
            tiling:      tiling.into(),
            offset:      offset.into_f32(),
            anchor_view: Rglica::default(),
        }
    }

    pub fn make(side: Anchor, offset: impl IntoF32) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.into_f32(),
            anchor_view: Rglica::default(),
        }
    }

    pub fn anchor(side: Anchor, offset: impl IntoF32, anchor_view: Rglica<dyn View>) -> Self {
        Self {
            side,
            tiling: None,
            offset: offset.into_f32(),
            anchor_view,
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
