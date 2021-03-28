
use crate::te::{Buffers, Shaders, Fonts};

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub fonts:   Fonts
}

impl Assets {
    pub fn init() -> Assets {
        Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            fonts:   Fonts::init()
        }
    }
}