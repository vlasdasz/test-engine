
extern crate gl;
extern crate glfw;

use crate::gm::*;

use glfw::{ Action, Context, Key, Glfw, WindowEvent, OpenGlProfileHint::Core, Window };

pub trait Updatable {
    fn update(&mut self);
}

pub struct GLLoader {
    pub window: Window,
    pub window_size: Size,
    pub events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
}

impl GLLoader {

    pub fn set_clear_color(&self, color: Color) {
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear(&self) {
        GL!(Clear, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
    }

    pub fn enable_depth_test(&self) {
        GL!(Enable, gl::DEPTH_TEST)
    }

    pub fn disable_depth_test(&self) {
        GL!(Disable, gl::DEPTH_TEST)
    }

    pub fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = 2.0;
        GL!(Viewport, (rect.origin.x * SCALE) as i32,
                      ((self.window_size.height - rect.origin.y - rect.size.height) * SCALE) as i32,
                      (rect.size.width * SCALE) as i32,
                      (rect.size.height * SCALE) as i32)
    }

    pub fn with_size(size: Size) -> GLLoader {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

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
        window.set_key_polling(true);

        GLLoader { window, window_size: size, events }
    }

}