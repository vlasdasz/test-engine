pub use buffer::{Buffer, BufferConfig};
use cfg_if::cfg_if;
pub use gl_info::GLInfo;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, MouseButton};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use gm::Point;
use gm::Size;
pub use shader::{Shader, ShaderCompiler};
use tools::New;

pub use crate::gl_wrapper::GLWrapper;

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

#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub trait DesktopInput {
    fn on_cursor_moved(&mut self, position: Point);
    fn on_mouse_click(&mut self, button: MouseButton, state: Action);
    fn on_key_pressed(&mut self, key: glfw::Key, action: glfw::Action);
}

#[cfg(any(target_os = "ios", target_os = "android"))]
pub trait DesktopInput {}

pub trait Screen: New + DesktopInput {
    fn update(&mut self);
    fn set_size(&mut self, size: Size) -> &mut Self;
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn start_main_loop(&mut self);
}
