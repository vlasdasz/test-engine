use rtools::{IntoF32, Rglica};

use crate::{
    layout::{layout_rule::LayoutRule, Anchor},
    View,
};

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

    pub fn width(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Width);
        self
    }

    pub fn height(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Height);
        self
    }

    pub fn center(&mut self) {
        self.rules.push(LayoutRule::make(Anchor::Center, 0))
    }

    pub fn offset(&mut self, offset: impl IntoF32) {
        let pending = self.pending_sides.drain(..);
        self.rules
            .extend(pending.map(|a| LayoutRule::make(a, offset.into_f32())))
    }

    pub fn anchor<T: View>(&mut self, view: Rglica<T>, offset: impl IntoF32) {
        debug_assert!(
            self.pending_sides.len() == 1,
            "Anchor shoud be to exactly one size"
        );
        let side = self.pending_sides.pop().unwrap();
        self.rules
            .push(LayoutRule::anchor(side, offset.into_f32(), view.rglica()));
    }
}
