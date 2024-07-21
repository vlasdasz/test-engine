use std::path::PathBuf;

use gm::Platform;

pub struct Paths;

impl Paths {
    pub fn storage() -> PathBuf {
        let home = if Platform::IOS {
            dirs::document_dir()
        } else if Platform::ANDROID {
            Some("idk_lol".into())
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
}
