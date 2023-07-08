#[derive(Debug)]
pub enum Tiling {
    Background,

    Horizontally,
    Vertically,

    Distribute(Vec<f32>),
}
