use std::{path::PathBuf, sync::OnceLock};

use audio::Sound;
use hreads::assert_main_thread;
use refs::manage::DataManager;
use window::{Font, image::Image};

use crate::assets_paths::AssetsPaths;

static ROOT_PATH: OnceLock<PathBuf> = OnceLock::new();

pub struct Assets;

impl Assets {
    pub(crate) fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let root_path = root_path.into();

        ROOT_PATH.set(root_path.clone()).expect("Double setting of root path");

        let paths = AssetsPaths::new(root_path);

        Image::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
        Font::set_root_path(&paths.fonts);
    }

    pub fn path() -> PathBuf {
        ROOT_PATH
            .get()
            .expect("Assets root path is not set yet")
            .as_path()
            .join("assets")
    }
}
