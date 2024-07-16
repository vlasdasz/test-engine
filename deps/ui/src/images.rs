use manage::data_manager::DataManager;
use refs::Weak;
use wgpu_wrapper::{image::Image, include_images};

pub struct UIImages;

include_images!(UIImages, "images");
