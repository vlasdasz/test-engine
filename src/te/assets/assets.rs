
use crate::te::{ Buffers, Shaders };

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders
}

impl Assets {
    pub fn init() -> Assets {
        Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init()
        }
    }
}