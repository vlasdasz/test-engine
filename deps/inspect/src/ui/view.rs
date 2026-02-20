use gm::flat::Rect;
use refs::Own;
use serde::{Deserialize, Serialize};
use ui::Placer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewRepr {
    pub label:    String,
    pub id:       String,
    pub frame:    Rect,
    pub placer:   Placer,
    pub subviews: Vec<Own<ViewRepr>>,
}

impl Default for ViewRepr {
    fn default() -> Self {
        Self {
            label:    String::default(),
            id:       String::default(),
            frame:    Rect::default(),
            placer:   Placer::empty(),
            subviews: vec![],
        }
    }
}
