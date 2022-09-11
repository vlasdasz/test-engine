#[cfg(desktop)]
use glfw::{Context, Window};
use gm::flat::Size;

use crate::{gl_events::GlEvents, gl_loader::GLFWEvents, monitor::Monitor, GLLoader};

pub struct GLFWManager {
    window:       Window,
    gl_events:    GLFWEvents,
    pub monitors: Vec<Monitor>,
}

impl GLFWManager {
    pub fn start_main_loop(&mut self) {
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            let events = GlEvents::get();

            for (_, event) in glfw::flush_messages(&self.gl_events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                        events.key_pressed.trigger((key, action))
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        events.cursor_moved.trigger((xpos, ypos).into())
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        events.size_changed.trigger((width, height).into())
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        events.mouse_click.trigger((btn, action))
                    }
                    _ => {}
                }
            }

            events.frame_drawn.trigger(());
            self.window.swap_buffers();
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.window.set_size(size.width as i32, size.height as i32)
    }
}

impl GLFWManager {
    pub fn new() -> Self {
        let mut loader = GLLoader::default();
        let monitors = loader.monitors();
        Self {
            window: loader.window,
            gl_events: loader.events,
            monitors,
        }
    }
}
