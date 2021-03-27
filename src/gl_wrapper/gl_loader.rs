
extern crate gl;
extern crate glfw;

use crate::gm::*;

use glfw::{ Context, WindowEvent, OpenGlProfileHint::Core, Window };

pub trait Updatable {
    fn new() -> Self;
    fn init(&mut self);
    fn set_size(&mut self, size: Size);
    fn update(&mut self);
}

pub struct GLLoader {
    pub window: Window,
    pub events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
}

impl GLLoader {

    pub fn with_size(size: Size) -> GLLoader {

        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(Some(16)));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }

        let (mut window, events) =
            glfw.create_window(size.width as u32,
                               size.height as u32,
                               "Hello this is window",
                               glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");

        GL!(load_with, |symbol| window.get_proc_address(symbol) as *const _);

        window.make_current();

        GLLoader { window, events }
    }

}