use cfg_if::cfg_if;

pub use buffer::{Buffer, BufferConfig};
pub use gl_info::GLInfo;
pub use gl_wrapper::GLWrapper;
pub use shader::{Shader, ShaderCompiler};

#[macro_use]
pub mod gl_debug;

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

cfg_if! {if #[cfg(not(any(target_os="ios", target_os="android")))] {
    pub mod gl_drawer;
    pub mod gl_loader;
    pub use gl_drawer::GLDrawer;
    pub use gl_loader::GLLoader;
}}

pub mod buffer;
pub mod gl_info;
pub mod gl_wrapper;
pub mod image_loader;
pub mod shader;
