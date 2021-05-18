#[macro_use]
pub mod gl_debug;

#[cfg(not(target_os = "ios"))]
pub mod gl_drawer;
#[cfg(not(target_os = "ios"))]
pub mod gl_loader;

#[cfg(not(target_os = "ios"))]
pub use gl_drawer::GLDrawer;
#[cfg(not(target_os = "ios"))]
pub use gl_loader::GLLoader;

pub mod buffer;
pub mod gl_info;
pub mod gl_wrapper;
pub mod shader;
pub mod texture;

pub use buffer::{Buffer, BufferConfig};
pub use gl_info::GLInfo;
pub use shader::{Shader, ShaderCompiler};

pub use gl_wrapper::GLWrapper;
pub use texture::TextureLoader;
