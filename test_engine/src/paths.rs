use std::path::PathBuf;

use home::home_dir;

pub fn home() -> PathBuf {
    home_dir().unwrap_or_default()
}

pub fn root() -> PathBuf {
    #[cfg(not(target_os = "ios"))]
    return home().join("game_name/test_engine");
    #[cfg(target_os = "android")]
    return Default::default();
    #[cfg(target_os = "ios")]
    return std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .unwrap()
        .to_path_buf();
}

pub fn assets() -> PathBuf {
    #[cfg(target_os = "android")]
    return Default::default();
    #[cfg(not(target_os = "android"))]
    return root().join("Assets");
}

pub fn images() -> PathBuf {
    assets().join("Images")
}
pub fn fonts() -> PathBuf {
    assets().join("Fonts")
}

pub mod shaders {
    use std::path::PathBuf;

    pub fn root() -> PathBuf {
        super::assets().join("Shaders")
    }

    pub fn ui() -> PathBuf {
        root().join("ui")
    }
    pub fn sprites() -> PathBuf {
        root().join("sprites")
    }
    pub fn isometric() -> PathBuf {
        root().join("isometric")
    }
    pub fn include() -> PathBuf {
        root().join("include")
    }
    pub fn test() -> PathBuf {
        root().join("test")
    }
}
