use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tiling {
    Background,

    Horizontally,
    Vertically,

    LeftHalf,
    RightHalf,

    Distribute(Vec<f32>),
}
