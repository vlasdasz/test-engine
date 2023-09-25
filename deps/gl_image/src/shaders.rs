use std::sync::OnceLock;

use anyhow::Result;
use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<ImageShaders> = OnceLock::new();

pub struct ImageShaders {
    color: Shader,
    mono:  Shader,
    text:  Shader,
}

impl ImageShaders {
    fn init() -> Result<Self> {
        let compiler = ShaderCompiler::new()?;

        let color_vert = include_str!("shaders/image_color.vert");
        let color_frag = include_str!("shaders/image_color.frag");

        let mono_vert = include_str!("shaders/image_mono.vert");
        let mono_frag = include_str!("shaders/image_mono.frag");

        let text_vert = include_str!("shaders/text_mono.vert");
        let text_frag = include_str!("shaders/text_mono.frag");

        let color = compiler.compile(color_vert, color_frag, "image_color")?;
        let mono = compiler.compile(mono_vert, mono_frag, "image_mono")?;
        let text = compiler.compile(text_vert, text_frag, "text_mono")?;

        Ok(Self { color, mono, text })
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(|| Self::init().unwrap())
    }

    pub fn color() -> &'static Shader {
        &Self::get().color
    }

    pub fn mono() -> &'static Shader {
        &Self::get().mono
    }

    pub fn text() -> &'static Shader {
        &Self::get().text
    }
}
