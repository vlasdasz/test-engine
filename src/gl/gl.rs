
use crate::gm;
extern crate glfw;

use gm::Size;
use glfw::{Action, Context, Key};

pub struct GL {

}

impl GL {

    pub fn init(size: Size) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) =
            glfw.create_window(size.width as u32,
                               size.height as u32,
                               "Hello this is window",
                               glfw::WindowMode::Windowed)

            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        while !window.should_close() {
            window.swap_buffers();

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