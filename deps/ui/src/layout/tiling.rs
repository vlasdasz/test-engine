use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Tiling {
    Background,

    Horizontally,
    Vertically,

    LeftHalf,
    RightHalf,

    Distribute(Vec<f32>),
}
