use gm::ToF32;

use super::Placer;
use crate::layout::{Anchor, layout_rule::LayoutRule};

impl Placer {
    pub fn t(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Top, offset.to_f32()));
        self
    }

    pub fn b(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Bot, offset.to_f32()));
        self
    }

    pub fn l(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Left, offset.to_f32()));
        self
    }

    pub fn r(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Right, offset.to_f32()));
        self
    }

    pub fn lr(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset)
    }

    pub fn tl(&self, offset: impl ToF32) -> &Self {
        self.t(offset).l(offset)
    }

    pub fn tr(&self, offset: impl ToF32) -> &Self {
        self.t(offset).r(offset)
    }

    pub fn bl(&self, offset: impl ToF32) -> &Self {
        self.b(offset).l(offset)
    }

    pub fn br(&self, offset: impl ToF32) -> &Self {
        self.b(offset).r(offset)
    }

    pub fn tb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).b(offset)
    }

    pub fn tlb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).l(offset).b(offset)
    }

    pub fn blt(&self, offset: impl ToF32) -> &Self {
        self.b(offset).l(offset).t(offset)
    }

    pub fn trb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).r(offset).b(offset)
    }

    pub fn lrt(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).t(offset)
    }

    pub fn tlr(&self, offset: impl ToF32) -> &Self {
        self.t(offset).l(offset).r(offset)
    }

    pub fn ltr(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).t(offset)
    }

    pub fn lrb(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).b(offset)
    }

    pub fn rbl(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).b(offset)
    }

    pub fn sides(&self, sides: &str, offset: impl ToF32) -> &Self {
        for ch in sides.chars() {
            match ch {
                't' => {
                    self.t(offset);
                }
                'b' => {
                    self.b(offset);
                }
                'l' => {
                    self.l(offset);
                }
                'r' => {
                    self.r(offset);
                }
                _ => panic!("Invalid side. Use letters tblr"),
            }
        }
        self
    }

    pub fn all_sides(&self, offset: impl ToF32) -> &Self {
        self.t(offset).b(offset).l(offset).r(offset)
    }
}
