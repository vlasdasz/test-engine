mod buffers;
mod shaders;

use std::path::Path;

pub use buffers::Buffers;
use gl_image::Image;
pub use shaders::Shaders;

use crate::paths;

#[derive(Default)]
pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
}

impl Assets {
    pub fn image(name: impl AsRef<Path>) -> Image {
        paths::images().join(name).into()
    }
}
