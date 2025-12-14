use std::{ path::PathBuf};

use filesystem::Paths;
use log::error;
use serde::Deserialize;

const CONFIG_YML: &str = include_str!("../../secrets/decrypted/test-game.yml");

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub sentry: String,
}

impl Config {
    pub fn sentry_url() -> Option<String> {
        let Ok(config) = serde_yaml::from_str::<Config>(&CONFIG_YML) else {
            error!("Failed to parse config toml from: {CONFIG_YML}");
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
