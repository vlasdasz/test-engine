use glfw::{Action, Key, MouseButton};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Context, Window};
use gm::{Point, Size};
use tools::{new, Event, New};

use crate::{gl_loader::GLFWEvents, GLLoader};

pub struct GLDrawer {
    window:              Window,
    events:              GLFWEvents,
    pub on_frame_drawn:  Event,
    pub on_cursor_moved: Event<Point>,
    pub on_size_changed: Event<Size>,
    pub on_mouse_click:  Event<(MouseButton, Action)>,
    pub on_key_pressed:  Event<(Key, Action)>,
}

impl GLDrawer {
    pub fn start_main_loop(&mut self) {
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                        self.on_key_pressed.trigger((key, action))
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.on_cursor_moved.trigger((xpos, ypos).into())
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        self.on_size_changed.trigger((width, height).into())
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        self.on_mouse_click.trigger((btn, action))
                    }
                    _ => {}
                }
            }

            self.on_frame_drawn.trigger(());
            self.window.swap_buffers();
        }
    }
}

impl New for GLDrawer {
    fn new() -> Self {
        let loader: GLLoader = new();
        Self {
            window:          loader.window,
            events:          loader.events,
            on_frame_drawn:  new(),
            on_cursor_moved: new(),
            on_size_changed: new(),
            on_mouse_click:  new(),
            on_key_pressed:  new(),
        }
    }
}
