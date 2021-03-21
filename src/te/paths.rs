
use std::env;
use std::path::PathBuf;

trait PathBufExt {
    fn pushing(&self, path: &str) -> PathBuf;
}

impl PathBufExt for PathBuf {
    fn pushing(&self, path: &str) -> PathBuf {
        let mut result = self.clone();
        result.push(path);
        return result;
    }
}

#[allow(deprecated)]
pub fn home() -> PathBuf { env::home_dir().unwrap_or_default() }
//pub fn root() -> PathBuf {  }
pub fn assets() -> PathBuf { home().pushing("Assets")  }

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