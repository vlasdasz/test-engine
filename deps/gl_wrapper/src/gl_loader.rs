#![cfg(desktop)]

extern crate glfw;

use std::os::raw::c_int;

use glfw::{
    ffi::{glfwSetWindowSizeCallback, GLFWwindow},
    Context, Glfw,
    OpenGlProfileHint::Core,
    SwapInterval, Window, WindowEvent,
};
use gm::flat::Size;

use crate::{monitor::Monitor, system_events::SystemEvents};

pub type GLFWEvents = std::sync::mpsc::Receiver<(f64, WindowEvent)>;

pub struct GLLoader {
    pub glfw:   Glfw,
    pub window: Window,
    pub events: GLFWEvents,
}

impl GLLoader {
    pub fn new(size: Size) -> Self {
        let mut glfw = glfw::init(|error, string| error!("GLFW error: {error} string: {string}")).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(16.into()));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        #[cfg(macos)]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(
                size.width as u32,
                size.height as u32,
                "Test Engine",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        unsafe { glfwSetWindowSizeCallback(window.window_ptr(), Some(size_callback)) };

        GL!(load_with, |symbol| window.get_proc_address(symbol).cast());

        window.make_current();

        // glfw.set_swap_interval(SwapInterval::None);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        GLLoader { glfw, window, events }
    }
}

impl GLLoader {
    pub fn monitors(&mut self) -> Vec<Monitor> {
        self.glfw
            .with_connected_monitors(|_, monitors| monitors.iter().map(Into::into).collect())
    }
}

extern "C" fn size_callback(_: *mut GLFWwindow, w: c_int, h: c_int) {
    SystemEvents::get().size_changed.trigger((w, h).into())
}
