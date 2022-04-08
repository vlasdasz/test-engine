mod buffers;
mod images;
mod shaders;

use std::{path::Path, rc::Rc};

pub use buffers::Buffers;
use rtools::Unwrap;
pub use shaders::Shaders;

use crate::{assets::images::Images, paths::Paths};

pub struct Assets {
    pub paths:   Rc<Paths>,
    pub images:  Unwrap<Images>,
    pub buffers: Unwrap<Buffers>,
    pub shaders: Unwrap<Shaders>,
}

impl Assets {
    pub fn new(root_path: &Path) -> Self {
        Self {
            paths:   Paths::new(root_path),
            images:  Default::default(),
            buffers: Default::default(),
            shaders: Default::default(),
        }
    }

    pub fn init_gl_data(&mut self) {
        self.buffers = Buffers::default().into();
        self.shaders = Shaders::new(&self.paths).into();
    }
}
