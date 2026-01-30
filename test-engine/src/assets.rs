use std::path::PathBuf;

use audio::Sound;
use hreads::assert_main_thread;
use refs::manage::DataManager;
use window::{Font, image::Image};

use crate::assets_paths::AssetsPaths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = AssetsPaths::new(root_path.into());

        Image::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
        Font::set_root_path(&paths.fonts);
    }
}
