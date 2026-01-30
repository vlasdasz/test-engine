use std::path::PathBuf;

use filesystem::Paths;
use log::warn;

use crate::App;

pub(crate) struct Config;

impl Config {
    pub async fn sentry_url(app: &dyn App) -> Option<String> {
        match app.sentry_url().await {
            Ok(url) => Some(url),
            Err(err) => {
                warn!("Failed to get sentry URL: {err}");
                None
            }
        }
    }
}

fn _config_path() -> PathBuf {
    let path = PathBuf::from("secrets/decrypted/test-game.yml");

    if let Ok(mut git_root) = Paths::git_root() {
        git_root.push(path);
        git_root
    } else {
        path
    }
}
