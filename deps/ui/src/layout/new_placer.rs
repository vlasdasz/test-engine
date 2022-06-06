use rtools::IntoF32;

use crate::layout::{layout_rule::LayoutRule, Anchor};

#[derive(Default)]
pub struct NewPlacer {
    pub(crate) rules: Vec<LayoutRule>,
    pending_sides:    Vec<Anchor>,
}

impl NewPlacer {
    pub fn top(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Top);
        self
    }

    pub fn bottom(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Bot);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Left);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Right);
        self
    }

    pub fn offset(&mut self, offset: impl IntoF32) {
        let pending = self.pending_sides.drain(..);
        self.rules
            .extend(pending.map(|a| LayoutRule::make(a, offset.into_f32())))
    }
}
