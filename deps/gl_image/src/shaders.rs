use std::sync::OnceLock;

use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<ImageShaders> = OnceLock::new();

pub struct ImageShaders {
    texture:    Shader,
    monochrome: Shader,
}

impl ImageShaders {
    fn init() -> Self {
        let compiler = ShaderCompiler::default();

        let ui_texture_vert = include_str!("shaders/ui_texture.vert");
        let ui_texture_frag = include_str!("shaders/ui_texture.frag");

        let monochrome_vert = include_str!("shaders/ui_monochrome.vert");
        let monochrome_frag = include_str!("shaders/ui_monochrome.frag");

        let texture = compiler.compile(ui_texture_vert, ui_texture_frag);
        let monochrome = compiler.compile(monochrome_vert, monochrome_frag);

        Self { texture, monochrome }
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(Self::init)
    }

    pub fn texture() -> &'static Shader {
        &Self::get().texture
    }

    pub fn monochrome() -> &'static Shader {
        &Self::get().monochrome
    }
}
