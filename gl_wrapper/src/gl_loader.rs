extern crate gl;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
extern crate glfw;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Context, OpenGlProfileHint::Core, Window, WindowEvent};
use tools::*;

pub type GLFWEvents = std::sync::mpsc::Receiver<(f64, WindowEvent)>;

pub struct GLLoader {
    pub window: Window,
    pub events: GLFWEvents,
}

impl New for GLLoader {
    fn new() -> GLLoader {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(Some(16)));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }

        let (mut window, events) = glfw
            .create_window(500, 500, "Test Engine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        GL!(load_with, |symbol| window.get_proc_address(symbol)
            as *const _);

        window.make_current();

        GLLoader { window, events }
    }
}
