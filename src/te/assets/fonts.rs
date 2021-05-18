use crate::te::paths;
use crate::ui::Font;

pub struct Fonts {
    pub default: Font,
}

impl Fonts {
    pub fn init() -> Fonts {
        Fonts {
            default: Font::new(&paths::fonts().join("SF.otf"), 24).unwrap(),
        }
    }
}
