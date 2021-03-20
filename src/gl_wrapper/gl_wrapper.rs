
use crate::gm;
extern crate gl;
extern crate glfw;

use gm::Size;
use gm::Color;
use glfw::{Action, Context, Key, Glfw };
use self::glfw::OpenGlProfileHint::Core;
//use gl::types::*;

pub struct GL;

impl GL {

    pub fn set_clear_color(color: Color) {
        unsafe { gl::ClearColor(color.r, color.g, color.b, color.a); }
    }

    pub fn clear() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
    }

    pub fn init(size: Size) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(Some(16)));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        let (mut window, events) =
            glfw.create_window(size.width as u32,
                               size.height as u32,
                               "Hello this is window",
                               glfw::WindowMode::Windowed)

            .expect("Failed to create GLFW window.");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        window.make_current();
        window.set_key_polling(true);

        while !window.should_close() {
            window.swap_buffers();

            GL::set_clear_color(Color::random());
            GL::clear();

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }

}