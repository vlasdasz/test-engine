use std::{path::PathBuf, sync::Mutex};

use gm::Platform;

static STORAGE_PATH: Mutex<Option<String>> = Mutex::new(None);

pub struct Paths;

impl Paths {
    pub fn storage() -> PathBuf {
        let home = if Platform::IOS {
            dirs::document_dir()
        } else if Platform::ANDROID {
            Some(
                STORAGE_PATH
                    .lock()
                    .unwrap()
                    .clone()
                    .expect("set storage path on android")
                    .into(),
            )
        } else {
            dirs::home_dir()
        }
        .expect("Failed to get home directory");

        format!("{}/.{}", home.display(), Self::executable_name()).into()
    }

    pub fn executable_name() -> String {
        std::env::current_exe()
            .expect("Failed to get std::env::current_exe()")
            .file_name()
            .expect("Failed to get executable name")
            .to_string_lossy()
            .into()
    }

    pub fn set_storage_path(path: String) {
        STORAGE_PATH.lock().unwrap().replace(path);
    }
}
