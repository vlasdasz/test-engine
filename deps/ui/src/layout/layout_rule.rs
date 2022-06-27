use rtools::{IntoF32, Rglica};

use crate::{layout::Anchor, View};

pub(crate) struct LayoutRule {
    pub(crate) side:   Anchor,
    pub(crate) offset: f32,

    pub(crate) anchor_view: Rglica<dyn View>,
}

impl LayoutRule {
    pub fn make(side: Anchor, offset: impl IntoF32) -> Self {
        Self {
            side,
            offset: offset.into_f32(),
            anchor_view: Rglica::default(),
        }
    }

    pub fn anchor(side: Anchor, offset: f32, anchor_view: Rglica<dyn View>) -> Self {
        Self {
            side,
            offset,
            anchor_view,
        }
    }
}

impl From<Anchor> for LayoutRule {
    fn from(anchor: Anchor) -> Self {
        Self::make(anchor, 0)
    }
}
