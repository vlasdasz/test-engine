use crate::te::{Buffers, Fonts, Images, Shaders};

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub images: Images,
    pub fonts: Fonts,
}

impl Assets {
    pub fn init() -> Assets {
        Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            images: Images::init(),
            fonts: Fonts::init(),
        }
    }
}
