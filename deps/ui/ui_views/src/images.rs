use gl_image::Image;
use rtools::data_manager::{DataManager, Handle};

pub struct Images;

impl Images {
    pub fn left() -> Handle<Image> {
        Image::load(include_bytes!("images/left.png"), "ui::default::left")
    }

    pub fn right() -> Handle<Image> {
        Image::load(include_bytes!("images/right.png"), "ui::default::right")
    }

    pub fn up() -> Handle<Image> {
        Image::load(include_bytes!("images/up.png"), "ui::default::up")
    }

    pub fn down() -> Handle<Image> {
        Image::load(include_bytes!("images/down.png"), "ui::default::down")
    }
}
