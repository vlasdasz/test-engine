use std::path::PathBuf;

use audio::Sound;
use manage::data_manager::DataManager;
use refs::assert_main_thread;
use window::image::Image;

use crate::assets_paths::AssetsPaths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = AssetsPaths::new(root_path.into());

        Image::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
    }
}
