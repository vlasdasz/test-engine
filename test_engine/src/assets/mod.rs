mod buffers;
mod shaders;

use std::{path::Path, rc::Rc};

use audio::Sound;
pub use buffers::Buffers;
use gl_image::Image;
use rtools::data_manager::DataManager;
pub use shaders::Shaders;
use ui::Font;

use crate::paths::Paths;

pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
    pub paths:   Rc<Paths>,
}

impl Assets {
    pub fn new(root_path: &Path) -> Rc<Self> {
        let paths = Paths::new(root_path);

        Image::set_path(&paths.images);
        Sound::set_path(&paths.sounds);
        Font::set_path(&paths.fonts);

        Rc::new(Self {
            buffers: Buffers::default(),
            shaders: Shaders::new(&paths),
            paths,
        })
    }

    pub fn init_gl_data(&mut self) {
        // self.buffers = Buffers::default().into();
        self.shaders = Shaders::new(&self.paths).into();
    }
}
