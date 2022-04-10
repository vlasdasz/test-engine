use std::{
    default::Default,
    path::{Path, PathBuf},
    ptr,
};

use rtools::data_manager::{DataManager, DataStorage, Managed};

use crate::Image;

static mut PATH: *const PathBuf = ptr::null_mut();
static mut STORAGE: *mut DataStorage<Image> = ptr::null_mut();

impl Managed for Image {}

impl DataManager<Image> for Image {
    fn path() -> &'static Path {
        unsafe { PATH.as_ref().unwrap() }
    }

    fn set_path(path: &Path) {
        unsafe {
            PATH = Box::into_raw(Box::new(path.to_path_buf()));
        }
    }

    fn storage() -> &'static mut DataStorage<Image> {
        unsafe {
            if STORAGE.is_null() {
                STORAGE = Box::into_raw(Box::new(Default::default()));
            }
            STORAGE.as_mut().unwrap()
        }
    }
}
