use std::sync::OnceLock;

use anyhow::Result;

use crate::{Shader, ShaderCompiler};

static SHADERS: OnceLock<BasicShaders> = OnceLock::new();

pub struct BasicShaders {
    path: Shader,
}

impl BasicShaders {
    fn init() -> Result<Self> {
        let compiler = ShaderCompiler::new()?;

        let path_vert = include_str!("path.vert");
        let path_frag = include_str!("path.frag");

        let path = compiler.compile(path_vert, path_frag, "basic_path")?;

        Ok(Self { path })
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(|| Self::init().unwrap())
    }

    pub fn path() -> &'static Shader {
        &Self::get().path
    }
}
