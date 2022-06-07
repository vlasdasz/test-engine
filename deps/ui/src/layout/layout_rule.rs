use gm::flat::Rect;
use rtools::Rglica;

use crate::{layout::Anchor, view::ViewFrame, View};

pub(crate) struct LayoutRule {
    side:   Anchor,
    offset: f32,
    view:   Rglica<dyn View>,
}

impl LayoutRule {
    pub fn make(side: Anchor, offset: f32) -> Self {
        Self {
            side,
            offset,
            view: Rglica::default(),
        }
    }

    pub fn anchor(side: Anchor, offset: f32, view: Rglica<dyn View>) -> Self {
        Self { side, offset, view }
    }

    pub fn layout(&self, frame: &mut Rect, s_frame: &Rect) {
        if self.view.is_ok() {
            self.anchor_layout(frame, self.view.frame())
        } else {
            self.simple_layout(frame, s_frame)
        }
    }
}

impl LayoutRule {
    fn simple_layout(&self, frame: &mut Rect, s_frame: &Rect) {
        match self.side {
            Anchor::Top => frame.origin.y = self.offset,
            Anchor::Bot => {
                frame.size.height = frame.height() + s_frame.height() - frame.max_y() - self.offset
            }
            Anchor::Left => frame.origin.x = self.offset,
            Anchor::Right => frame.size.width = frame.width() + s_frame.width() - frame.max_x() - self.offset,
            Anchor::Width => frame.size.width = self.offset,
            Anchor::Height => frame.size.height = self.offset,
            _ => (),
        }
    }

    fn anchor_layout(&self, frame: &mut Rect, a_frame: &Rect) {
        match self.side {
            Anchor::Top => frame.origin.y = a_frame.max_y() + self.offset,
            _ => unreachable!("Not implemented yet"),
        }
    }
}
