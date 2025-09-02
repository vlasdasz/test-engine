use std::path::PathBuf;

use refs::{assert_main_thread, manage::DataManager};
use window::image::Image;

use crate::assets_paths::AssetsPaths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = AssetsPaths::new(root_path.into());

        Image::set_root_path(&paths.images);
        audio::Sound::set_root_path(&paths.sounds);
    }
}
