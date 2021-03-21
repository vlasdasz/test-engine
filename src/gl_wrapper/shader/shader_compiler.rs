
use std::path::PathBuf;

use crate::gl_wrapper::Shader;
use crate::gl_wrapper::gl_info::GLInfo;

pub struct ShaderCompiler {
    pub gl_info: GLInfo
}

impl ShaderCompiler {

    fn version(&self) -> String {
        let mut result = "#version ".to_string();

        result += &self.gl_info.glsl_version;

        if self.gl_info.is_gles {
            result += " es";
        }
        else {
            result += " core";
        }

        result + "\n"
    }

    pub fn compile(&self, _path: PathBuf) -> Shader {
        Shader { program: 10 }
    }

}
