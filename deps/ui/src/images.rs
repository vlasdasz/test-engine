use refs::Weak;
use window::image::Image;

pub struct UIImages;

impl UIImages {
    pub fn delete() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/delete.png"), "delete.png")
    }

    pub fn up() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/up.png"), "up.png")
    }

    pub fn down() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/down.png"), "down.png")
    }

    pub fn left() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/left.png"), "left.png")
    }

    pub fn right() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/right.png"), "right.png")
    }

    pub fn rb() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/rb.png"), "rb.png")
    }

    pub fn plus() -> Weak<Image> {
        Image::from_file_data(include_bytes!("images/plus.png"), "plus.png")
    }
}
