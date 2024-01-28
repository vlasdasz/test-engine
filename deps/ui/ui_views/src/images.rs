use gl_image::GlImage;

pub struct UIImages;
use manage::data_manager::DataManager;
use refs::Weak;

impl UIImages {
    pub fn plus() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/plus.png"), "ui::default::plus")
    }

    pub fn minus() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/minus.png"), "ui::default::minus")
    }

    pub fn left() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/left.png"), "ui::default::left")
    }

    pub fn right() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/right.png"), "ui::default::right")
    }

    pub fn up() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/up.png"), "ui::default::up")
    }

    pub fn down() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/down.png"), "ui::default::down")
    }

    pub fn dot() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/dot.png"), "ui::default::dot")
    }

    pub fn delete() -> Weak<GlImage> {
        GlImage::load(include_bytes!("images/delete.png"), "ui::default::delete")
    }
}
