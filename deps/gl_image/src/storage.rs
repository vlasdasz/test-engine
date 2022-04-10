use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
    sync::Mutex,
};

use lazy_static::lazy_static;
use rtools::data_manager::{DataManager, DataStorage};

use crate::Image;

type ImageStorage = DataStorage<Image>;

lazy_static! {
    static ref PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    static ref IMAGE_STORAGE: Mutex<ImageStorage> = Mutex::new(ImageStorage::default());
}

impl DataManager<Image> for Image {
    fn path() -> PathBuf {
        PATH.lock().unwrap().clone()
    }

    fn set_path(path: &Path) {
        let mut p = PATH.lock().unwrap();
        *p = path.into();
    }

    fn storage(a: &mut dyn FnMut(&mut ImageStorage)) {
        let mut storage = IMAGE_STORAGE.lock().unwrap();
        a(storage.deref_mut());
    }
}
