use std::{
    collections::HashMap,
    ffi::CString,
    path::{Path, PathBuf},
};

use anyhow::Result;
use rtools::{
    file::File,
    regex::{find_match, find_matches},
};

#[cfg(mobile)]
use crate::gl_debug::*;
use crate::{GLInfo, Shader};

pub struct ShaderCompiler {
    gl_info: GLInfo,
}

impl ShaderCompiler {
    pub fn new() -> Result<Self> {
        Ok(ShaderCompiler {
            gl_info: GLInfo::new()?,
        })
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

    fn alloc_str(len: usize) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        buffer.extend([b' '].iter().cycle().take(len));
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    #[allow(clippy::cast_sign_loss)]
    fn check_program_error(program: u32) {
        let mut success: GLT!(GLint) = 1;

        GL_SILENT!(GetShaderiv, program, GLC!(COMPILE_STATUS), &mut success);
        //         ^ returns invalid errors everywhere
        GL!(GetError);

        if success != 0 {
            return;
        }

        let mut len: GLT!(GLint) = 0;

        GL!(GetShaderiv, program, GLC!(INFO_LOG_LENGTH), &mut len);

        let error = Self::alloc_str(len as usize);

        GL!(
            GetShaderInfoLog,
            program,
            len,
            std::ptr::null_mut(),
            error.as_ptr().cast_mut()
        );

        let error = error.to_string_lossy().into_owned();
        panic!("Failed to compile shader: {error}");
    }

    fn unfold_includes(mut code: String) -> Result<String> {
        const QUOTES_QUERY: &str = r#"(("[^ "]+"))"#;
        const INCLUDE_QUERY: &str = r#"#include (("[^ "]+"))"#;
        let includes = find_matches(&code, INCLUDE_QUERY)?;
        let mut files: HashMap<String, String> = HashMap::new();
        for include in includes {
            let file_name = find_match(&include, QUOTES_QUERY)?.replace('\"', "");
            let file_path = PathBuf::default().join(file_name);
            let include_code = File::read_to_string(file_path);
            files.insert(include, include_code);
        }
        for (include, include_code) in files {
            code = code.replace(include.as_str(), include_code.as_str());
        }

        Ok(code)
    }

    fn compile_shader(&self, code: impl ToString, kind: GLT!(GLenum)) -> Result<u32> {
        let code = self.version() + "\n" + &Self::unfold_includes(code.to_string())?;
        let shader = GL!(CreateShader, kind);

        let c_code = CString::new(code).unwrap();

        let code_ptr = c_code.as_ptr();
        GL!(ShaderSource, shader, 1, &code_ptr, std::ptr::null());
        GL!(CompileShader, shader);

        Self::check_program_error(shader);

        Ok(shader)
    }

    pub fn compile(
        &self,
        vert_code: impl ToString,
        frag_code: impl ToString,
        name: impl ToString,
    ) -> Result<Shader> {
        let vert = self.compile_shader(vert_code, GLC!(VERTEX_SHADER))?;
        let frag = self.compile_shader(frag_code, GLC!(FRAGMENT_SHADER))?;

        let program = GL!(CreateProgram);

        GL!(AttachShader, program, vert);
        GL!(AttachShader, program, frag);
        GL!(LinkProgram, program);

        Self::check_program_error(program);

        GL!(DetachShader, program, vert);
        GL!(DetachShader, program, frag);

        GL!(DeleteShader, vert);
        GL!(DeleteShader, frag);

        trace!("Shader: OK");

        Ok(Shader::new(program, name))
    }

    pub fn compile_path(&self, path: &Path) -> Result<Shader> {
        trace!("Compiling: {:?}", path);

        let vert_path = path.with_extension("vert");
        let frag_path = path.with_extension("frag");

        let vert_code = File::read_to_string(vert_path);
        let frag_code = File::read_to_string(frag_path);

        self.compile(vert_code, frag_code, path.display())
    }
}
