use gl_wrapper::{Shader, ShaderCompiler};
use rtools::static_init;

pub struct ImageShaders {
    pub(crate) texture:    Shader,
    pub(crate) monochrome: Shader,
}

impl Default for ImageShaders {
    fn default() -> Self {
        let compiler = ShaderCompiler::default();

        let ui_texture_vert = include_str!("shaders/ui_texture.vert");
        let ui_texture_frag = include_str!("shaders/ui_texture.frag");

        let monochrome_vert = include_str!("shaders/ui_monochrome.vert");
        let monochrome_frag = include_str!("shaders/ui_monochrome.frag");

        let texture = compiler.compile(ui_texture_vert, ui_texture_frag);
        let monochrome = compiler.compile(monochrome_vert, monochrome_frag);

        Self { texture, monochrome }
    }
}

static_init!(ImageShaders);
