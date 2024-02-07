use std::path::PathBuf;

use audio::Sound;
use manage::data_manager::DataManager;
use ui::refs::assert_main_thread;
use wgpu_wrapper::image::Image;

use crate::paths::Paths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = Paths::new(root_path.into());

        Image::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
    }
}
