
use crate::te::paths;

use crate::gl_wrapper::Shader;
use crate::gl_wrapper::ShaderCompiler;
use crate::te::paths::PathBufExt;

pub struct Shaders {
    compiler: ShaderCompiler,

    pub ui:            Shader,
    pub ui_path:       Shader,
    pub ui_texture:    Shader,
    pub ui_monochrome: Shader
}

impl Shaders {
    pub fn init() -> Shaders {

        let compiler = ShaderCompiler::new();

        let ui            = compiler.compile(paths::shaders::ui().pushing("ui"));
        let ui_path       = compiler.compile(paths::shaders::ui().pushing("ui_path"));
        let ui_texture    = compiler.compile(paths::shaders::ui().pushing("ui_texture"));
        let ui_monochrome = compiler.compile(paths::shaders::ui().pushing("ui_monochrome"));

        Shaders {
            compiler,

            ui,
            ui_path,
            ui_texture,
            ui_monochrome
        }
    }
}