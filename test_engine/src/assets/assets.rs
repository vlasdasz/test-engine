use std::rc::Rc;

use crate::assets::{Buffers, Fonts, Images, Shaders};

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub images:  Images,
    pub fonts:   Fonts,
}

impl Assets {
    pub fn init() -> Rc<Assets> {
        Rc::new(Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            images:  Images::init(),
            fonts:   Fonts::init(),
        })
    }
}
