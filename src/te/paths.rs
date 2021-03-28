
use std::env;
use std::path::PathBuf;

pub trait PathBufExt {
    fn pushing<P: AsRef<str>>(&self, path: P) -> PathBuf;
}

impl PathBufExt for PathBuf {
    fn pushing<P: AsRef<str>>(&self, path: P) -> PathBuf {
        let mut result = self.clone();
        result.push(path.as_ref());
        return result;
    }
}

#[allow(deprecated)]
pub fn home() -> PathBuf { env::home_dir().unwrap_or_default() }
pub fn root() -> PathBuf { home().pushing(".deps/test_engine") }
pub fn assets() -> PathBuf { root().pushing("Assets")  }

pub fn fonts() -> PathBuf { assets().pushing("Fonts")  }

pub mod shaders {
    use std::path::PathBuf;
    use crate::te::paths::PathBufExt;

    pub fn root() -> PathBuf { super::assets().pushing("Shaders") }

    pub fn ui       () -> PathBuf { root().pushing("ui")        }
    pub fn sprites  () -> PathBuf { root().pushing("sprites")   }
    pub fn isometric() -> PathBuf { root().pushing("isometric") }
    pub fn include  () -> PathBuf { root().pushing("include")   }
    pub fn test     () -> PathBuf { root().pushing("test")      }
}