#![cfg(desktop)]
#![allow(clippy::mismatched_target_os)]

extern crate glfw;

use glfw::{Context, Glfw, OpenGlProfileHint::Core, Window, WindowEvent};

use crate::monitor::Monitor;

pub type GLFWEvents = std::sync::mpsc::Receiver<(f64, WindowEvent)>;

pub struct GLLoader {
    pub glfw:   Glfw,
    pub window: Window,
    pub events: GLFWEvents,
}

impl Default for GLLoader {
    fn default() -> Self {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(16.into()));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        #[cfg(macos)]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(500, 500, "Test Engine", glfw::WindowMode::Windowed)
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
}

impl GLLoader {
    pub fn monitors(&mut self) -> Vec<Monitor> {
        self.glfw
            .with_connected_monitors(|_, monitors| monitors.iter().map(|a| a.into()).collect())
    }
}
