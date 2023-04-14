use gl_wrapper::{Shader, ShaderCompiler};

use crate::paths::Paths;

pub struct Shaders {
    pub sprite:       Shader,
    pub image_sprite: Shader,
}

impl Shaders {
    pub fn new(paths: &Paths) -> Shaders {
        trace!("Compiling shaders");

        let compiler = ShaderCompiler::new(&paths.shaders.include);

        trace!("ShaderCompiler: Ok");

        let sprite = compiler.compile_path(&paths.shaders.sprites.join("sprite"));
        let image_sprite = compiler.compile_path(&paths.shaders.sprites.join("textured_sprite"));

        trace!("Shaders: OK");

        Shaders { sprite, image_sprite }
    }
}
