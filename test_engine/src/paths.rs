use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use home::home_dir;

pub fn home() -> PathBuf {
    home_dir().unwrap()
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
