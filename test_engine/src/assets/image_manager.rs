use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use gl_image::Image;
use lazy_static::lazy_static;

type ImageStorage = HashMap<String, Image>;

lazy_static! {
    static ref PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    static ref IMAGE_STORAGE: Mutex<ImageStorage> = Mutex::new(ImageStorage::default());
}

pub trait ImageManager {
    fn set_path(path: &Path) {
        let mut p = PATH.lock().unwrap();
        *p = path.into();
    }

    fn get(path: &str) -> Image;
}

fn path() -> PathBuf {
    PATH.lock().unwrap().clone()
}

impl ImageManager for Image {
    fn get(name: &str) -> Image {
        let mut storage = IMAGE_STORAGE.lock().unwrap();

        if storage.contains_key(name) {
            return storage[name].clone();
        }

        let image = Image::load(path().join(name));
        storage.insert(name.into(), image.clone());
        image
    }
}
