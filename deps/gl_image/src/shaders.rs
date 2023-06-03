use std::sync::OnceLock;

use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<ImageShaders> = OnceLock::new();

pub struct ImageShaders {
    color: Shader,
    mono:  Shader,
}

impl ImageShaders {
    fn init() -> Self {
        let compiler = ShaderCompiler::default();

        let color_vert = include_str!("shaders/image_color.vert");
        let color_frag = include_str!("shaders/image_color.frag");

        let mono_vert = include_str!("shaders/image_mono.vert");
        let mono_frag = include_str!("shaders/image_mono.frag");

        let color = compiler.compile(color_vert, color_frag, "image_color");
        let mono = compiler.compile(mono_vert, mono_frag, "image_mono");

        Self { color, mono }
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(Self::init)
    }

    pub fn color() -> &'static Shader {
        &Self::get().color
    }

    pub fn mono() -> &'static Shader {
        &Self::get().mono
    }
}
