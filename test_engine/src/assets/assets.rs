use tools::New;

use crate::assets::{Buffers, Fonts, Images, Shaders};

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub images:  Images,
    pub fonts:   Fonts,
}

impl New for Assets {
    fn new() -> Assets {
        Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            images:  Images::init(),
            fonts:   Fonts::init(),
        }
    }
}
