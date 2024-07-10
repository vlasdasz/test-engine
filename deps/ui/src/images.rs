use wgpu_wrapper::image::Image;

pub struct UIImages;
use manage::data_manager::DataManager;
use refs::Weak;

impl UIImages {
    pub fn plus() -> Weak<Image> {
        Image::load(include_bytes!("images/plus.png"), "ui::default::plus")
    }

    pub fn minus() -> Weak<Image> {
        Image::load(include_bytes!("images/minus.png"), "ui::default::minus")
    }

    pub fn left() -> Weak<Image> {
        Image::load(include_bytes!("images/left.png"), "ui::default::left")
    }

    pub fn right() -> Weak<Image> {
        Image::load(include_bytes!("images/right.png"), "ui::default::right")
    }

    pub fn up() -> Weak<Image> {
        Image::load(include_bytes!("images/up.png"), "ui::default::up")
    }

    pub fn down() -> Weak<Image> {
        Image::load(include_bytes!("images/down.png"), "ui::default::down")
    }

    pub fn dot() -> Weak<Image> {
        Image::load(include_bytes!("images/dot.png"), "ui::default::dot")
    }

    pub fn delete() -> Weak<Image> {
        Image::load(include_bytes!("images/delete.png"), "ui::default::delete")
    }

    pub fn rb_corner() -> Weak<Image> {
        Image::load(include_bytes!("images/rb.png"), "ui::default::rb_corner")
    }
}
