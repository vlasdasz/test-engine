#![allow(incomplete_features)]
#![feature(specialization)]

mod image;
mod shaders;
mod to_image;

pub use shaders::ImageShaders;
pub use to_image::*;

pub use self::image::{draw_image, GlImage};
