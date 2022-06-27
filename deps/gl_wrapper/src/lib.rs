#![allow(clippy::mismatched_target_os)]

pub use buffers::{Buffer, BufferConfig};
use cfg_if::cfg_if;
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

cfg_if! { if #[cfg(desktop)] {
    pub mod glfw_manager;
    pub mod gl_loader;
    pub use glfw_manager::GLFWManager;
    pub use gl_loader::GLLoader;
}}

pub mod buffers;
pub mod gl_info;
pub mod gl_wrapper;
pub mod global_events;
pub mod image_loader;
pub mod monitor;
pub mod shaders;
