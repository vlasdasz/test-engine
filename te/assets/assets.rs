use crate::te::{Buffers, Fonts, Images, Shaders};
use std::rc::Rc;

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub images: Images,
    pub fonts: Fonts,
}

impl Assets {
    pub fn init() -> Rc<Assets> {
        Rc::new(Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            images: Images::init(),
            fonts: Fonts::init(),
        })
    }
}
