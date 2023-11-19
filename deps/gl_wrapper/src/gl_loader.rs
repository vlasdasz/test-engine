#![cfg(desktop)]

extern crate glfw;

use std::{ffi::c_int, mem::transmute};

use glfw::{
    ffi::{glfwSetWindowSizeCallback, GLFWwindow},
    Context, Glfw, GlfwReceiver,
    OpenGlProfileHint::Core,
    SwapInterval, Window, WindowEvent,
};
use gm::flat::IntSize;

use crate::{monitor::Monitor, system_events::SystemEvents};

pub type GLFWEvents = GlfwReceiver<(f64, WindowEvent)>;

pub struct GLLoader {
    pub glfw:   Glfw,
    pub window: Window,
    pub events: GLFWEvents,
}

impl GLLoader {
    pub fn new(size: IntSize) -> Self {
        let mut glfw = glfw::init(|error, string| error!("GLFW error: {error} string: {string}")).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(16.into()));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        #[cfg(macos)]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (window, events) = glfw
            .create_window(size.width, size.height, "Test Engine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        fn unbox<T>(value: Box<T>) -> T {
            Box::into_inner(value)
        }

        let window: Box<Window> = unsafe { transmute(window) };

        let mut window = unbox(window);

        unsafe { glfwSetWindowSizeCallback(window.window_ptr(), Some(window_size_callback)) };

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

extern "C" fn window_size_callback(_: *mut GLFWwindow, w: c_int, h: c_int) {
    #[allow(clippy::cast_sign_loss)]
    SystemEvents::get().size_changed.trigger((w as u32, h as u32).into())
}
