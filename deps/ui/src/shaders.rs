use gl_wrapper::{Shader, ShaderCompiler};
use rtools::static_init;

pub struct UIShaders {
    pub(crate) view: Shader,
    pub(crate) path: Shader,
}

impl UIShaders {
    pub fn view() -> &'static Shader {
        &Self::get().view
    }

    pub fn path() -> &'static Shader {
        &Self::get().path
    }
}

impl Default for UIShaders {
    fn default() -> Self {
        let compiler = ShaderCompiler::default();

        let view_vert = include_str!("shaders/ui.vert");
        let view_frag = include_str!("shaders/ui.frag");

        let path_vert = include_str!("shaders/ui_path.vert");
        let path_frag = include_str!("shaders/ui_path.frag");

        let view = compiler.compile(view_vert, view_frag);
        let path = compiler.compile(path_vert, path_frag);

        Self { view, path }
    }
}

static_init!(UIShaders);
