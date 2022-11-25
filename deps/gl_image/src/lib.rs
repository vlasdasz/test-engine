#![allow(incomplete_features)]
#![feature(specialization)]

mod image;
mod shaders;

pub use shaders::ImageShaders;

pub use self::image::{draw_image, Image};
