mod buffers;
mod fonts;
mod images;
mod shaders;

use tools::New;

pub use buffers::Buffers;
pub use fonts::Fonts;
pub use images::Images;
pub use shaders::Shaders;

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub images: Images,
    pub fonts: Fonts,
}

impl New for Assets {
    fn new() -> Assets {
        Assets {
            buffers: Buffers::init(),
            shaders: Shaders::init(),
            images: Images::init(),
            fonts: Fonts::init(),
        }
    }
}
