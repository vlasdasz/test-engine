use crate::ui::Font;
use crate::te::paths;

pub struct Fonts {
    pub default: Font
}

impl Fonts {
    pub fn init() -> Fonts {
        Fonts { default: Font::new(&paths::fonts().join("SF.otf"), 24).unwrap() }
    }
}