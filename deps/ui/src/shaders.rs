use std::sync::OnceLock;

use anyhow::Result;
use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<UIShaders> = OnceLock::new();

pub struct UIShaders {
    view: Shader,
}

impl UIShaders {
    fn init() -> Result<Self> {
        let compiler = ShaderCompiler::new()?;

        let view_vert = include_str!("shaders/ui.vert");
        let view_frag = include_str!("shaders/ui.frag");

        let view = compiler.compile(view_vert, view_frag, "ui")?;
        Ok(Self { view })
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(|| Self::init().unwrap())
    }

    pub fn view() -> &'static Shader {
        &Self::get().view
    }
}
