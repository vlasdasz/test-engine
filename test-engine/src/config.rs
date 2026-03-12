#[cfg(not(target_arch = "wasm32"))]
fn _config_path() -> std::path::PathBuf {
    let path = std::path::PathBuf::from("secrets/decrypted/test-game.yml");

    if let Ok(mut git_root) = filesystem::Paths::git_root() {
        git_root.push(path);
        git_root
    } else {
        path
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct Config;

#[cfg(not(target_arch = "wasm32"))]
impl Config {
    pub async fn sentry_url(app: &dyn crate::App) -> Option<String> {
        match app.sentry_url().await {
            Ok(url) => Some(url),
            Err(err) => {
                log::warn!("Failed to get sentry URL: {err}");
                None
            }
        }
    }
}
