#[derive(Debug)]
pub enum Tiling {
    Background,

    Horizontally,
    Vertically,

    LeftHalf,
    RightHalf,

    Distribute(Vec<f32>),
}
