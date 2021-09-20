use gl_wrapper::{Shader, ShaderCompiler};

use crate::paths;

pub struct Shaders {
    compiler: ShaderCompiler,

    pub ui: Shader,
    pub ui_path: Shader,
    pub ui_texture: Shader,
    pub ui_monochrome: Shader,

    pub sprite: Shader,
    pub textured_sprite: Shader,
}

impl Shaders {
    pub fn init() -> Shaders {
        let compiler = ShaderCompiler::new(paths::shaders::include());

        let ui = compiler.compile(&paths::shaders::ui().join("ui"));
        let ui_path = compiler.compile(&paths::shaders::ui().join("ui_path"));
        let ui_texture = compiler.compile(&paths::shaders::ui().join("ui_texture"));
        let ui_monochrome = compiler.compile(&paths::shaders::ui().join("ui_monochrome"));

        let sprite = compiler.compile(&paths::shaders::sprites().join("sprite"));
        let textured_sprite = compiler.compile(&paths::shaders::sprites().join("textured_sprite"));

        Shaders {
            compiler,

            ui,
            ui_path,
            ui_texture,
            ui_monochrome,

            sprite,
            textured_sprite,
        }
    }
}
