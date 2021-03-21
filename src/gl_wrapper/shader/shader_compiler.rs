
extern crate gl;

use std::fs;
use std::path::PathBuf;

use crate::log;
use crate::te::paths;
use crate::gl_wrapper::Shader;
use crate::gl_wrapper::gl_info::GLInfo;

use crate::utils::regex::*;
use std::collections::HashMap;
use crate::te::paths::PathBufExt;
use std::ffi::CStr;

pub struct ShaderCompiler {
    pub gl_info: GLInfo
}

impl ShaderCompiler {

    const QUOTES_QUERY: &'static str = r#"(("[^ "]+"))"#;
    const INCLUDE_QUERY: &'static str = r#"#include (("[^ "]+"))"#;

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

    fn unfold_includes(code: &mut String) {
        let includes = find_matches(&code, ShaderCompiler::INCLUDE_QUERY);
        let mut files: HashMap<String, String> = HashMap::new();
        for include in includes {
            let mut file_name = find_match(&include, ShaderCompiler::QUOTES_QUERY);
            file_name = file_name.replace("\"", "");
            let file_path = paths::shaders::include().pushing(file_name);
            let include_code = fs::read_to_string(file_path).unwrap();
            files.insert(include, include_code);
        }
        for (include, include_code) in files {
            *code = code.replace(include.as_str(), include_code.as_str());
        }
    }

    // auto shader = GL(glCreateShader(type));
    // auto code_pointer = code.c_str();
    // GL(glShaderSource(shader, 1, &code_pointer, nullptr));
    // GL(glCompileShader(shader));
    // check_programm_error(file_name, shader, code);
    // return shader;
    // }

    fn compile_shader(&self, path: PathBuf, mut code: String, kind: gl::types::GLenum) -> u32 {

        ShaderCompiler::unfold_includes(&mut code);

        code = self.version() + "\n" + &code;

        let shader = unsafe {
            gl::CreateShader(kind)
        };

        unsafe {
          //  gl::ShaderSource(shader, 1, CStr::from(&code).as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        println!("{}", code);


        return 5;
    }

    pub fn compile(&self, path: PathBuf) -> Shader {

        log(&ShaderCompiler::INCLUDE_QUERY);

        let vert_path = path.with_extension("vert");
        let frag_path = path.with_extension("frag");

        let vert_code = fs::read_to_string(&vert_path).unwrap();
        let frag_code = fs::read_to_string(&frag_path).unwrap();

        println!("{:?}", vert_code);
        println!("{:?}", frag_code);

        let vert = self.compile_shader(vert_path, vert_code, gl::VERTEX_SHADER);
        let frag = self.compile_shader(frag_path, frag_code, gl::FRAGMENT_SHADER);

        Shader { program: 10 }
    }

}
