mod buffers;
mod shaders;

use std::{path::Path, rc::Rc};

use audio::Sound;
pub use buffers::Buffers;
use gl_image::Image;
use rtools::{data_manager::DataManager, Unwrap};
pub use shaders::Shaders;
use ui::Font;

use crate::paths::Paths;

pub struct Assets {
    pub paths:   Rc<Paths>,
    pub buffers: Unwrap<Buffers>,
    pub shaders: Unwrap<Shaders>,
}

impl Assets {
    pub fn new(root_path: &Path) -> Self {
        let paths = Paths::new(root_path);

        Image::set_path(&paths.images);
        Sound::set_path(&paths.sounds);
        Font::set_path(&paths.fonts);

        Self {
            paths,
            buffers: Default::default(),
            shaders: Default::default(),
        }
    }

    pub fn init_gl_data(&mut self) {
        self.buffers = Buffers::default().into();
        error!("Buffers: OK");
        self.shaders = Shaders::new(&self.paths).into();
        error!("Shaders: OK");
    }
}
