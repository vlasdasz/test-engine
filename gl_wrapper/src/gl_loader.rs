#![cfg(not(any(target_os = "ios", target_os = "android")))]

extern crate glfw;

use glfw::{Context, Glfw, OpenGlProfileHint::Core, Window, WindowEvent};
use gm::Size;

use crate::monitor::Monitor;

pub type GLFWEvents = std::sync::mpsc::Receiver<(f64, WindowEvent)>;

pub struct GLLoader {
    pub glfw:   Glfw,
    pub window: Window,
    pub events: GLFWEvents,
}

impl GLLoader {
    pub fn new(size: Size) -> GLLoader {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(Some(16)));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(
                size.width as u32,
                size.height as u32,
                "Test Engine",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        GL!(load_with, |symbol| window.get_proc_address(symbol)
            as *const _);

        window.make_current();

        GLLoader {
            glfw,
            window,
            events,
        }
    }

    pub fn monitors(&mut self) -> Vec<Monitor> {
        self.glfw
            .with_connected_monitors(|_, monitors| monitors.iter().map(|a| a.into()).collect())
    }
}
