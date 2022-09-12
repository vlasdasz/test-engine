use gl_wrapper::{Shader, ShaderCompiler};

use crate::paths::Paths;

pub struct Shaders {
    pub ui:      Shader,
    pub ui_path: Shader,

    pub sprite:       Shader,
    pub image_sprite: Shader,
}

impl Shaders {
    pub fn new(paths: &Paths) -> Shaders {
        trace!("Compiling shaders");

        let compiler = ShaderCompiler::new(&paths.shaders.include);

        trace!("ShaderCompiler: Ok");

        let ui = compiler.compile_path(&paths.shaders.ui.join("ui"));
        let ui_path = compiler.compile_path(&paths.shaders.ui.join("ui_path"));

        let sprite = compiler.compile_path(&paths.shaders.sprites.join("sprite"));
        let image_sprite = compiler.compile_path(&paths.shaders.sprites.join("textured_sprite"));

        trace!("Shaders: OK");

        Shaders {
            ui,
            ui_path,

            sprite,
            image_sprite,
        }
    }
}
