mod shaders;

use std::{path::PathBuf, ptr::null_mut, rc::Rc};

use audio::Sound;
use gl_image::Image;
use rtools::data_manager::DataManager;
pub use shaders::Shaders;
use text::Font;
use ui::refs::is_main_thread;

use crate::paths::Paths;

static mut ASSETS: *const Assets = null_mut();

pub struct Assets {
    pub shaders: Shaders,
    pub paths:   Rc<Paths>,
}

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert!(is_main_thread());

        let paths = Paths::new(root_path.into());

        Image::set_path(&paths.images);
        Sound::set_path(&paths.sounds);
        Font::set_path(&paths.fonts);

        unsafe {
            ASSETS = Box::into_raw(Box::new(Self {
                shaders: Shaders::new(&paths),
                paths,
            }));
        }
    }

    pub fn get() -> &'static Assets {
        assert!(is_main_thread());
        unsafe {
            assert!(!ASSETS.is_null(), "Assets were not initialized");
            ASSETS.as_ref().unwrap()
        }
    }
}
