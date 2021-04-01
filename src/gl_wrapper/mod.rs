
#[macro_use] pub mod gl_debug;

#[cfg(not(target_os="ios"))]
pub mod gl_loader;
#[cfg(not(target_os="ios"))]
pub mod gl_drawer;

#[cfg(not(target_os="ios"))]
pub use gl_loader::GLLoader;
#[cfg(not(target_os="ios"))]
pub use gl_drawer::GLDrawer;

pub mod buffer;
pub mod shader;
pub mod gl_info;
pub mod gl_wrapper;
pub mod texture;

pub use buffer::{ Buffer, BufferConfig };
pub use shader::{ Shader, ShaderCompiler };
pub use gl_info::GLInfo;

pub use gl_wrapper::GLWrapper;
pub use texture::TextureLoader;