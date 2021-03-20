
extern crate gl;

pub struct GLInfo {
    pub is_gles: bool,
    pub version: String,
    pub major_version: u8,
    pub glsl_version: String,
    pub glsl_version_number: u8,
}

impl GLInfo {
    pub fn get() -> GLInfo {
        unsafe {
            use std::str;
            //use std::libc::c_char;
            use std::ffi::CStr;

            let full_gl_version = gl::GetString(gl::VERSION);
            //let c_str: &CStr = unsafe { CStr::from_ptr(full_gl_version) };
           // println!("{}", c_str.to_str().unwrap());
        }

        return GLInfo {
            is_gles: false,
            version: String::from("0"),
            major_version: 0,
            glsl_version: String::from("a"),
            glsl_version_number: 0
        }
    }
}