use refs::{Weak, manage::DataManager};

use crate::image::Image;

pub struct NoImage;

pub trait ToImage {
    fn to_image(&self) -> Weak<Image>;
}

impl ToImage for Weak<Image> {
    fn to_image(&self) -> Weak<Image> {
        *self
    }
}

impl ToImage for String {
    fn to_image(&self) -> Weak<Image> {
        Image::get(self)
    }
}

impl ToImage for &str {
    fn to_image(&self) -> Weak<Image> {
        Image::get(self)
    }
}

impl ToImage for NoImage {
    fn to_image(&self) -> Weak<Image> {
        Weak::default()
    }
}
