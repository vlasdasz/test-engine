#[cfg(feature = "2d")]
mod _2d {
    pub type Point = gm::flat::Point;
    pub type Size = gm::flat::Size;
    pub type Rotation = f32;
}

#[cfg(feature = "3d")]
mod _3d {
    pub type Point = gm::volume::Point3;
    pub type Size = gm::volume::Size3;
    pub type Point = gm::volume::Quaternion;
}

#[cfg(feature = "2d")]
pub use _2d::*;
#[cfg(feature = "3d")]
pub use _3d::*;
