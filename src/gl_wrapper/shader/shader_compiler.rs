
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

    pub fn compile(&self, path: PathBuf) -> Shader {

        use std::fs;

        let vertex_path = path.with_extension("vert");

        println!("{:?}", vertex_path);

        let vertex_code = fs::read_to_string(path.with_extension("vert"));

        println!("{:?}", vertex_code);

        Shader { program: 10 }
    }

}
