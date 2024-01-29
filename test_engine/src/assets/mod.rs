use std::path::PathBuf;

use audio::Sound;
use gl_image::GlImage;
use manage::data_manager::DataManager;
use text::GlFont;
use ui::refs::assert_main_thread;
use wgpu_wrapper::image::Image;

use crate::paths::Paths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = Paths::new(root_path.into());

        dbg!(&paths.images);

        Image::set_root_path(&paths.images);
        GlImage::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
        GlFont::set_root_path(&paths.fonts);
    }
}
