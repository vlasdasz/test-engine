use gm::flat::Rect;
use rtools::Rglica;

use crate::{layout::Anchor, View};

pub(crate) struct LayoutRule {
    side:   Anchor,
    offset: f32,
    _view:  Rglica<dyn View>,
}

impl LayoutRule {
    pub fn make(side: Anchor, offset: f32) -> Self {
        Self {
            side,
            offset,
            _view: Rglica::default(),
        }
    }

    pub fn layout(&self, frame: &mut Rect, s_frame: &Rect) {
        match self.side {
            Anchor::Top => frame.origin.y = self.offset,
            Anchor::Bot => {
                frame.size.height = frame.height() + s_frame.height() - frame.max_y() - self.offset;
            }
            Anchor::Left => frame.origin.x = self.offset,
            Anchor::Right => {
                frame.size.width = frame.width() + s_frame.width() - frame.max_x() - self.offset;
            }
            _ => (),
        };
    }
}
