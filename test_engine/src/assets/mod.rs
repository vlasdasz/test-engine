mod buffers;
mod shaders;

use std::{path::Path, rc::Rc};

pub use buffers::Buffers;
use gl_image::Image;
use rtools::{data_manager::DataManager, Unwrap};
pub use shaders::Shaders;

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

        Self {
            paths,
            buffers: Default::default(),
            shaders: Default::default(),
        }
    }

    pub fn init_gl_data(&mut self) {
        self.buffers = Buffers::default().into();
        self.shaders = Shaders::new(&self.paths).into();
    }
}
