use gl_image::Image;

pub struct UIImages;
use manage::{data_manager::DataManager, handle::Handle};

impl UIImages {
    pub fn plus() -> Handle<Image> {
        Image::load(include_bytes!("images/plus.png"), "ui::default::plus")
    }

    pub fn minus() -> Handle<Image> {
        Image::load(include_bytes!("images/minus.png"), "ui::default::minus")
    }

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

    pub fn dot() -> Handle<Image> {
        Image::load(include_bytes!("images/dot.png"), "ui::default::dot")
    }

    pub fn delete() -> Handle<Image> {
        Image::load(include_bytes!("images/delete.png"), "ui::default::delete")
    }
}
