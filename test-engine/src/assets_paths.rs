use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

pub(crate) struct AssetsPaths {
    pub(crate) images: PathBuf,
    pub(crate) sounds: PathBuf,
}

impl AssetsPaths {
    pub fn new(root: PathBuf) -> Rc<Self> {
        let root = Self::root(&root);
        let assets = Self::assets(&root);
        Rc::new(Self {
            images: assets.join("images"),
            sounds: assets.join("sounds"),
        })
    }
}

#[allow(clippy::used_underscore_binding)]
impl AssetsPaths {
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
        return _root.join("assets");
    }
}
