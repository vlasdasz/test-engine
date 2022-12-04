use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use home::home_dir;

pub fn home() -> PathBuf {
    home_dir().unwrap()
}

pub struct Paths {
    pub assets:  PathBuf,
    pub images:  PathBuf,
    pub sounds:  PathBuf,
    pub fonts:   PathBuf,
    pub shaders: ShaderPaths,
}

impl Paths {
    pub fn new(root: PathBuf) -> Rc<Self> {
        let root = Self::root(&root);
        let assets = Self::assets(&root);
        Rc::new(Self {
            assets:  assets.clone(),
            images:  assets.join("Images"),
            sounds:  assets.join("Sounds"),
            fonts:   assets.join("Fonts"),
            shaders: ShaderPaths::with_assets(&assets),
        })
    }
}

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

pub struct ShaderPaths {
    pub ui:        PathBuf,
    pub sprites:   PathBuf,
    pub isometric: PathBuf,
    pub include:   PathBuf,
    pub test:      PathBuf,
}

impl ShaderPaths {
    fn with_assets(assets: &Path) -> Self {
        let root = assets.join("Shaders");
        Self {
            ui:        root.join("ui"),
            sprites:   root.join("sprites"),
            isometric: root.join("isometric"),
            include:   root.join("include"),
            test:      root.join("test"),
        }
    }
}
