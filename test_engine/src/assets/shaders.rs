use gl_wrapper::{Shader, ShaderCompiler};

use crate::paths::Paths;

pub struct Shaders {
    pub ui:            Shader,
    pub ui_path:       Shader,
    pub ui_texture:    Shader,
    pub ui_monochrome: Shader,

    pub sprite:       Shader,
    pub image_sprite: Shader,
}

impl Shaders {
    pub fn new(paths: &Paths) -> Shaders {
        error!("Creating shaders");

        let compiler = ShaderCompiler::new(&paths.shaders.include);

        error!("created compiler");

        let ui = compiler.compile(&paths.shaders.ui.join("ui"));
        let ui_path = compiler.compile(&paths.shaders.ui.join("ui_path"));
        let ui_texture = compiler.compile(&paths.shaders.ui.join("ui_texture"));
        let ui_monochrome = compiler.compile(&paths.shaders.ui.join("ui_monochrome"));

        let sprite = compiler.compile(&paths.shaders.sprites.join("sprite"));
        let image_sprite = compiler.compile(&paths.shaders.sprites.join("textured_sprite"));

        error!("shaders created");

        Shaders {
            ui,
            ui_path,
            ui_texture,
            ui_monochrome,

            sprite,
            image_sprite,
        }
    }
}
