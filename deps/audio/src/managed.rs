use std::{
    path::{Path, PathBuf},
    ptr,
};

use rtools::data_manager::{DataManager, DataStorage, Managed};

use crate::Sound;

static mut PATH: *const PathBuf = ptr::null_mut();
static mut STORAGE: *mut DataStorage<Sound> = ptr::null_mut();

impl Managed for Sound {}

impl DataManager<Sound> for Sound {
    fn path() -> &'static Path {
        unsafe { PATH.as_ref().unwrap() }
    }

    fn set_path(path: &Path) {
        unsafe {
            PATH = Box::into_raw(Box::new(path.to_path_buf()));
        }
    }

    fn storage() -> &'static mut DataStorage<Sound> {
        unsafe {
            if STORAGE.is_null() {
                STORAGE = Box::into_raw(Box::new(Default::default()));
            }
            STORAGE.as_mut().unwrap()
        }
    }
}
