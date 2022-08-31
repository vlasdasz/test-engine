use gm::flat::{Rect, Size};
use rtools::{IntoF32, Rglica};

use crate::{
    layout::{layout_rule::LayoutRule, Anchor},
    view::ViewFrame,
    View,
};

#[derive(Default)]
pub struct NewPlacer {
    pub(crate) rules: Vec<LayoutRule>,
    pending_sides:    Vec<Anchor>,

    has_width:  bool,
    has_height: bool,
}

impl NewPlacer {
    pub fn background(&mut self) -> &mut Self {
        self.top().bottom().left().right()
    }

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

    pub fn size(&mut self, size: impl Into<Size>) -> &mut Self {
        let size = size.into();
        self.width(size.width).height(size.height)
    }

    pub fn width(&mut self, w: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Width, w));
        self.has_width = true;
        self
    }

    pub fn height(&mut self, h: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Height, h));
        self.has_height = true;
        self
    }

    pub fn center(&mut self) -> &mut Self {
        self.rules.push(Anchor::Center.into());
        self
    }

    pub fn center_hor(&mut self) -> &mut Self {
        self.rules.push(Anchor::CenterH.into());
        self
    }

    pub fn center_ver(&mut self) -> &mut Self {
        self.rules.push(Anchor::CenterV.into());
        self
    }

    pub fn as_background(&mut self) -> &mut Self {
        self.rules.push(Anchor::Background.into());
        self
    }

    pub fn offset(&mut self, offset: impl IntoF32) -> &mut Self {
        let pending = self.pending_sides.drain(..);
        self.rules
            .extend(pending.map(|a| LayoutRule::make(a, offset.into_f32())));
        self
    }

    pub fn anchor<T: View>(&mut self, view: Rglica<T>, offset: impl IntoF32) {
        debug_assert!(
            self.pending_sides.len() == 1,
            "Anchor should be to exactly one size"
        );
        let side = self.pending_sides.pop().unwrap();
        self.rules
            .push(LayoutRule::anchor(side, offset.into_f32(), view.rglica()));
    }

    fn assign_pending(&mut self) {
        let pending = self.pending_sides.drain(..);
        self.rules.extend(pending.map(|a| a.into()))
    }
}

impl NewPlacer {
    pub fn layout(&mut self, frame: &mut Rect, s_frame: &Rect) {
        self.assign_pending();
        for rule in &self.rules {
            if rule.anchor_view.is_ok() {
                self.anchor_layout(rule, frame, rule.anchor_view.frame())
            } else {
                self.simple_layout(rule, frame, s_frame)
            }
        }
    }
}

impl NewPlacer {
    fn simple_layout(&self, rule: &LayoutRule, frame: &mut Rect, s_frame: &Rect) {
        match rule.side {
            Anchor::Top => frame.origin.y = rule.offset,
            Anchor::Bot => {
                if self.has_height {
                    frame.origin.y = s_frame.height() - frame.height() - rule.offset
                } else {
                    frame.size.height = frame.height() + s_frame.height() - frame.max_y() - rule.offset
                }
            }
            Anchor::Left => frame.origin.x = rule.offset,
            Anchor::Right => {
                if self.has_width {
                    frame.origin.x = s_frame.width() - frame.width() - rule.offset;
                } else {
                    frame.size.width = frame.width() + s_frame.width() - frame.max_x() - rule.offset
                }
            }
            Anchor::Width => frame.size.width = rule.offset,
            Anchor::Height => frame.size.height = rule.offset,
            Anchor::CenterH => frame.origin.x = s_frame.width() / 2.0 - frame.width() / 2.0,
            Anchor::CenterV => frame.origin.y = s_frame.height() / 2.0 - frame.height() / 2.0,
            Anchor::Center => {
                frame.origin.x = s_frame.width() / 2.0 - frame.width() / 2.0;
                frame.origin.y = s_frame.height() / 2.0 - frame.height() / 2.0;
            }
            Anchor::Background => *frame = s_frame.with_zero_origin(),
        }
    }

    fn anchor_layout(&self, rule: &LayoutRule, frame: &mut Rect, a_frame: &Rect) {
        match rule.side {
            Anchor::Top => frame.origin.y = a_frame.max_y() + rule.offset,
            _ => unreachable!("Not implemented yet"),
        }
    }
}
