use std::default::default;

use glfw::{Action, Key, MouseButton};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Context, Window};
use gm::{Point, Size};
use tools::Event;

use crate::{gl_loader::GLFWEvents, monitor::Monitor, GLLoader};

pub struct GLDrawer {
    window:              Window,
    events:              GLFWEvents,
    pub monitors:        Vec<Monitor>,
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

    pub fn set_size(&mut self, size: Size) {
        self.window.set_size(size.width as i32, size.height as i32)
    }

    pub fn monitor(&self) -> &Monitor {
        self.monitors.first().unwrap_or_else(|| {
            error!("Failed to get monitor");
            panic!();
        })
    }

    pub fn scale(&self) -> f32 {
        self.monitor().scale
    }
}

impl GLDrawer {
    pub fn new(size: Size) -> Self {
        error!("Creating GLDrawer");

        let loader = GLLoader::new(size);
        Self {
            window:   loader.window,
            events:   loader.events,
            monitors: loader.monitors,

            on_frame_drawn:  default(),
            on_cursor_moved: default(),
            on_size_changed: default(),
            on_mouse_click:  default(),
            on_key_pressed:  default(),
        }
    }
}
