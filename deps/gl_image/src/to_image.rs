use manage::data_manager::DataManager;
use refs::Weak;

use crate::GlImage;

pub trait ToImage {
    fn to_image(&self) -> Weak<GlImage>;
}

impl ToImage for Weak<GlImage> {
    fn to_image(&self) -> Weak<GlImage> {
        *self
    }
}

impl ToImage for String {
    fn to_image(&self) -> Weak<GlImage> {
        GlImage::get(self)
    }
}

impl ToImage for &str {
    fn to_image(&self) -> Weak<GlImage> {
        GlImage::get(self)
    }
}
