use gm::flat::Rect;
use serde::{Deserialize, Serialize};
use ui::Placer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewRepr {
    pub label:    String,
    pub frame:    Rect,
    pub placer:   Option<Placer>,
    pub subviews: Vec<ViewRepr>,
}
