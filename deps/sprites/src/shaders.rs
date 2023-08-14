use std::sync::OnceLock;

use gl_wrapper::{Shader, ShaderCompiler};

static SHADERS: OnceLock<SpriteShaders> = OnceLock::new();

pub struct SpriteShaders {
    sprite:          Shader,
    textured_sprite: Shader,
}

impl SpriteShaders {
    fn init() -> Self {
        let compiler = ShaderCompiler::default();

        let sprite_vert = include_str!("shaders/sprite.vert");
        let sprite_frag = include_str!("shaders/sprite.frag");

        let textured_sprite_vert = include_str!("shaders/textured_sprite.vert");
        let textured_sprite_frag = include_str!("shaders/textured_sprite.frag");

        let sprite = compiler.compile(sprite_vert, sprite_frag, "sprite");
        let textured_sprite = compiler.compile(textured_sprite_vert, textured_sprite_frag, "textured_sprite");

        Self {
            sprite,
            textured_sprite,
        }
    }

    fn get() -> &'static Self {
        SHADERS.get_or_init(Self::init)
    }

    pub fn sprite() -> &'static Shader {
        &Self::get().sprite
    }

    pub fn textured_sprite() -> &'static Shader {
        &Self::get().textured_sprite
    }
}
