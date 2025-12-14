use std::path::PathBuf;

use filesystem::Paths;
use log::error;
use serde::Deserialize;

use crate::App;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub sentry: String,
}

impl Config {
    pub fn sentry_url(app: &dyn App) -> Option<String> {
        let Some(config) = app.config_yaml() else {
            error!("No config yaml");
            return None;
        };

        let Ok(config) = serde_yaml::from_str::<Config>(&config) else {
            error!("Failed to parse config from: {config}");
            return None;
        };

        config.sentry.into()
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
