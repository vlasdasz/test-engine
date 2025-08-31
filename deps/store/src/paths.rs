use std::{path::PathBuf, process::Command, sync::Mutex};

use dirs::home_dir;
use log::warn;
use plat::Platform;

static STORAGE_PATH: Mutex<Option<String>> = Mutex::new(None);

pub struct Paths;

impl Paths {
    pub fn storage() -> PathBuf {
        #[cfg(target_arch = "wasm32")]
        {
            return PathBuf::default();
        }

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

    pub fn home() -> PathBuf {
        home_dir().unwrap()
    }

    pub fn git_root() -> anyhow::Result<PathBuf> {
        // let output = Command::new("git").args(["rev-parse",
        // "--show-toplevel"]).output()?;
        //
        // if !output.status.success() {
        //     warn!("Failed to get Git repository root path");
        //     return Ok(PathBuf::from("~/dev/money"));
        // }
        //
        // assert!(output.status.success(), "Failed to get Git repository root path");
        // let git_root =
        // String::from_utf8_lossy(&output.stdout).trim_end_matches('\n').to_string();
        //
        // Ok(PathBuf::from(git_root))

        Ok(PathBuf::new())
    }

    pub fn set_storage_path(path: String) {
        STORAGE_PATH.lock().unwrap().replace(path);
    }
}
