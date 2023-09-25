use std::sync::OnceLock;

use anyhow::Result;
use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<UIShaders> = OnceLock::new();

pub struct UIShaders {
    view: Shader,
    path: Shader,
}

impl UIShaders {
    fn init() -> Result<Self> {
        let compiler = ShaderCompiler::new()?;

        let view_vert = include_str!("shaders/ui.vert");
        let view_frag = include_str!("shaders/ui.frag");

        let path_vert = include_str!("shaders/ui_path.vert");
        let path_frag = include_str!("shaders/ui_path.frag");

        let view = compiler.compile(view_vert, view_frag, "ui")?;
        let path = compiler.compile(path_vert, path_frag, "ui_path")?;

        Ok(Self { view, path })
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(|| Self::init().unwrap())
    }

    pub fn view() -> &'static Shader {
        &Self::get().view
    }

    pub fn path() -> &'static Shader {
        &Self::get().path
    }
}
