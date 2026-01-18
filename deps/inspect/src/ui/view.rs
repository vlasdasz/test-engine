use gm::flat::Rect;
use serde::{Deserialize, Serialize};
use ui::Placer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewRepr {
    pub label:    String,
    pub frame:    Rect,
    pub placer:   Placer,
    pub subviews: Vec<ViewRepr>,
}

impl Default for ViewRepr {
    fn default() -> Self {
        Self {
            label:    String::default(),
            frame:    Rect::default(),
            placer:   Placer::empty(),
            subviews: vec![],
        }
    }
}
