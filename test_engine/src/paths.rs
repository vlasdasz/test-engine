use std::{
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
};

use anyhow::Result;
use home::home_dir;

pub fn home() -> PathBuf {
    home_dir().unwrap()
}

pub fn git_root() -> Result<PathBuf> {
    let output = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?;
    assert!(output.status.success(), "Failed to get Git repository root path");
    let git_root = String::from_utf8_lossy(&output.stdout).trim_end_matches('\n').to_string();

    Ok(PathBuf::from(git_root))
}

pub(crate) struct Paths {
    pub(crate) images: PathBuf,
    pub(crate) sounds: PathBuf,
    pub(crate) fonts:  PathBuf,
}

impl Paths {
    pub fn new(root: PathBuf) -> Rc<Self> {
        let root = Self::root(&root);
        let assets = Self::assets(&root);
        Rc::new(Self {
            images: assets.join("Images"),
            sounds: assets.join("Sounds"),
            fonts:  assets.join("Fonts"),
        })
    }
}

#[allow(clippy::used_underscore_binding)]
impl Paths {
    fn root(_base: &Path) -> PathBuf {
        #[cfg(ios)]
        return std::env::current_exe().unwrap_or_default().parent().unwrap().to_path_buf();
        #[cfg(not(ios))]
        return _base.into();
    }

    pub fn assets(_root: &Path) -> PathBuf {
        #[cfg(android)]
        return Default::default();
        #[cfg(not(android))]
        return _root.join("Assets");
    }
}
