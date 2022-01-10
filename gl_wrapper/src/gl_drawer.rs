#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Context, Window};
use gm::Size;
use tools::Rglica;

use crate::{events::Events, gl_loader::GLFWEvents, monitor::Monitor, GLLoader};

pub struct GLDrawer {
    window:       Window,
    gl_events:    GLFWEvents,
    events:       Rglica<Events>,
    pub monitors: Vec<Monitor>,
}

impl GLDrawer {
    pub fn start_main_loop(&mut self) {
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.gl_events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                        self.events.on_key_pressed.trigger((key, action))
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.events.on_cursor_moved.trigger((xpos, ypos).into())
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        self.events.on_size_changed.trigger((width, height).into())
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        self.events.on_mouse_click.trigger((btn, action))
                    }
                    _ => {}
                }
            }

            self.events.on_frame_drawn.trigger(());
            self.window.swap_buffers();
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.window.set_size(size.width as i32, size.height as i32)
    }
}

impl GLDrawer {
    pub fn new(size: Size, events: Rglica<Events>) -> Self {
        error!("Creating GLDrawer");

        let mut loader = GLLoader::new(size);
        let monitors = loader.monitors();
        Self {
            window: loader.window,
            gl_events: loader.events,
            events,

            monitors,
        }
    }
}
