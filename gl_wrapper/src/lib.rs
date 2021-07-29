use cfg_if::cfg_if;

pub use buffer::{Buffer, BufferConfig};
pub use gl_info::GLInfo;
pub use gl_wrapper::GLWrapper;
pub use shader::{Shader, ShaderCompiler};
use gm::{Size, Point};
#[cfg(not(any(target_os="ios", target_os="android")))]
use glfw::{MouseButton, Action};
use tools::New;

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

pub trait Screen: New {
    fn init(&mut self);
    fn update(&mut self);
    fn set_size(&mut self, size: Size);
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn on_cursor_moved(&mut self, position: Point);
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn on_mouse_key_pressed(&self, button: MouseButton, state: Action);
}
