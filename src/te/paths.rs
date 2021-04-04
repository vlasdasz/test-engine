
use std::env;
use std::path::PathBuf;

#[allow(deprecated)]
pub fn home() -> PathBuf { env::home_dir().unwrap_or_default() }
pub fn root() -> PathBuf { home().join(".deps/test_engine") }
pub fn assets() -> PathBuf { root().join("Assets")  }

pub fn images() -> PathBuf { assets().join("Images") }
pub fn fonts () -> PathBuf { assets().join("Fonts" ) }

pub mod shaders {
    use std::path::PathBuf;

    pub fn root() -> PathBuf { super::assets().join("Shaders") }

    pub fn ui       () -> PathBuf { root().join("ui"       ) }
    pub fn sprites  () -> PathBuf { root().join("sprites"  ) }
    pub fn isometric() -> PathBuf { root().join("isometric") }
    pub fn include  () -> PathBuf { root().join("include"  ) }
    pub fn test     () -> PathBuf { root().join("test"     ) }
}