use gm::flat::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewRepr {
    pub label:    String,
    pub frame:    Rect,
    pub subviews: Vec<ViewRepr>,
}
