use crate::ui::Font;
use crate::te::paths;
use crate::te::paths::PathBufExt;

pub struct Fonts {
    pub default: Font
}

impl Fonts {
    pub fn init() -> Fonts {
        Fonts { default: Font::new(&paths::fonts().pushing("SF.otf"), 24).unwrap() }
    }
}