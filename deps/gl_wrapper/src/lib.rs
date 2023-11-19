#![feature(const_trait_impl)]
#![feature(process_exitcode_internals)]
#![feature(box_into_inner)]

pub use buffers::{Buffer, BufferConfig};
pub use gl_info::GLInfo;
pub use shaders::{Shader, ShaderCompiler};

pub use crate::gl_wrapper::GLWrapper;

#[macro_use]
pub mod gl_debug;

#[cfg(mobile)]
#[macro_use]
extern crate mashup;

#[macro_use]
extern crate log;

#[cfg(desktop)]
pub mod gl_loader;
#[cfg(desktop)]
pub mod glfw_manager;
#[cfg(desktop)]
pub use gl_loader::GLLoader;
#[cfg(desktop)]
pub use glfw_manager::GLFWManager;

pub mod buffers;
pub mod gl_info;
pub mod gl_wrapper;
pub mod image_loader;
pub mod monitor;
pub mod shaders;
pub mod system_events;
