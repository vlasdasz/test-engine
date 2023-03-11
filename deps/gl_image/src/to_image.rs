use rtools::data_manager::{DataManager, Handle};

use crate::Image;

pub trait ToImage {
    fn to_image(&self) -> Handle<Image>;
}

impl ToImage for Handle<Image> {
    fn to_image(&self) -> Handle<Image> {
        *self
    }
}

impl ToImage for String {
    fn to_image(&self) -> Handle<Image> {
        Image::get(self)
    }
}

impl ToImage for &str {
    fn to_image(&self) -> Handle<Image> {
        Image::get(self)
    }
}
