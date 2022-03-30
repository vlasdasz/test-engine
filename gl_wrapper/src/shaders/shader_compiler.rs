use cfg_if::cfg_if;

cfg_if! { if #[cfg(any(target_os="ios", target_os="android"))] {
    use gles31_sys::*;
} else {
    extern crate gl;
}}

use std::{
    collections::HashMap,
    ffi::CString,
    path::{Path, PathBuf},
};

use rtools::{
    file::File,
    regex::{find_match, find_matches},
};

use crate::{GLInfo, Shader};

pub struct ShaderCompiler {
    path:    PathBuf,
    gl_info: GLInfo,
}

impl ShaderCompiler {
    pub fn new(path: PathBuf) -> ShaderCompiler {
        ShaderCompiler {
            path,
            gl_info: GLInfo::get(),
        }
    }

    fn version(&self) -> String {
        let mut result = "#version ".to_string();
        result += &self.gl_info.glsl_version;
        if self.gl_info.is_gles {
            result += " es";
        } else {
            result += " core";
        }
        result + "\n"
    }

    fn check_programm_error(&self, path: &Path, program: u32) {
        let mut success: GLT!(GLint) = 1;

        GL_SILENT!(GetShaderiv, program, GLC!(COMPILE_STATUS), &mut success);
        //         ^ returns invalid errors everywhere
        GL!(GetError);

        if success != 0 {
            return;
        }

        let mut len: GLT!(GLint) = 0;

        GL!(GetShaderiv, program, GLC!(INFO_LOG_LENGTH), &mut len);

        fn alloc_str(len: usize) -> CString {
            let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
            buffer.extend([b' '].iter().cycle().take(len));
            unsafe { CString::from_vec_unchecked(buffer) }
        }

        let error = alloc_str(len as usize);

        GL!(
            GetShaderInfoLog,
            program,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLT!(GLchar)
        );

        let error = error.to_string_lossy().into_owned();
        error!("Failed to compile shader: {:?} error: {}", path, error);
        error!("Spilkok {}", error);
        panic!("Failed to compile shader: {:?} error: {}", path, error);
    }

    fn unfold_includes(&self, mut code: String) -> String {
        const QUOTES_QUERY: &str = r#"(("[^ "]+"))"#;
        const INCLUDE_QUERY: &str = r#"#include (("[^ "]+"))"#;
        let includes = find_matches(&code, INCLUDE_QUERY);
        let mut files: HashMap<String, String> = HashMap::new();
        for include in includes {
            let file_name = find_match(&include, QUOTES_QUERY).replace('\"', "");
            let file_path = self.path.join(file_name);
            let include_code = File::read_to_string(file_path);
            files.insert(include, include_code);
        }
        for (include, include_code) in files {
            code = code.replace(include.as_str(), include_code.as_str());
        }
        code
    }

    fn compile_shader(&self, path: PathBuf, code: String, kind: GLT!(GLenum)) -> u32 {
        let code = self.version() + "\n" + &self.unfold_includes(code);
        let shader = GL!(CreateShader, kind);

        let c_code = CString::new(code).unwrap();

        error!("code: {:?}", c_code);

        let code_ptr = c_code.as_ptr();
        GL!(ShaderSource, shader, 1, &code_ptr, std::ptr::null());
        GL!(CompileShader, shader);

        self.check_programm_error(&path, shader);

        shader
    }

    pub fn compile(&self, path: &Path) -> Shader {
        let vert_path = path.with_extension("vert");
        let frag_path = path.with_extension("frag");

        let vert_code = File::read_to_string(&vert_path);
        let frag_code = File::read_to_string(&frag_path);

        let vert = self.compile_shader(vert_path, vert_code, GLC!(VERTEX_SHADER));
        let frag = self.compile_shader(frag_path, frag_code, GLC!(FRAGMENT_SHADER));

        let program = GL!(CreateProgram);

        GL!(AttachShader, program, vert);
        GL!(AttachShader, program, frag);
        GL!(LinkProgram, program);

        self.check_programm_error(path, program);

        GL!(DetachShader, program, vert);
        GL!(DetachShader, program, frag);

        GL!(DeleteShader, vert);
        GL!(DeleteShader, frag);

        Shader::new(program, path.to_string_lossy().into())
    }
}
