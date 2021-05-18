#[cfg(target_os = "ios")]
use gles31_sys::*;

#[cfg(not(target_os = "ios"))]
extern crate gl;

use tools::regex::find_match;

#[derive(Debug)]
pub struct GLInfo {
    pub is_gles: bool,
    pub gl_version: String,
    pub major_version: u8,
    pub glsl_version: String,
    pub glsl_version_number: u16,
}

impl GLInfo {
    const GLSL_QUERY: &'static str = r#"((\d\.)(\d\d))"#;
    const GL_QUERY: &'static str = r#"((\d\.)(\d))"#;

    fn get_string(id: u32) -> String {
        use std::ffi::CStr;
        let c_str: &CStr = unsafe {
            let full_gl_version = GL!(GetString, id);
            cfg_if::cfg_if! {
                if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
                    type CPath = *const u8;
                }
                else {
                    type CPath = *const i8;
                }
            };
            CStr::from_ptr(full_gl_version as CPath)
        };
        c_str.to_str().unwrap().to_string()
    }

    pub fn get() -> GLInfo {
        let version = GLInfo::get_string(GLC!(VERSION));
        let is_gles = version.contains("ES");
        let gl_version = find_match(&version, GLInfo::GL_QUERY);
        let mut glsl_version = gl_version.clone();
        glsl_version = glsl_version.replace(".", "");
        glsl_version += "0";
        let glsl_version_number = glsl_version.parse::<u16>().unwrap();
        let ch = glsl_version.chars().next().unwrap();
        let major_version = ch.to_string().parse::<u8>().unwrap();

        return GLInfo {
            is_gles,
            gl_version,
            major_version,
            glsl_version,
            glsl_version_number,
        };
    }
}
