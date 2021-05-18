use crate::gl_wrapper::GLLoader;
use crate::gm::{Point, Size};

use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::ui::input::touch::{ButtonState, MouseButton};
use glfw::{Context, Window, WindowEvent};

pub struct GLDrawer<T: Updatable> {
    window: Window,
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
    drawer: T,
}

impl<T: Updatable> GLDrawer<T> {
    pub fn with_size(size: Size) -> GLDrawer<T> {
        let loader = GLLoader::with_size(size);
        let mut drawer = T::new();
        drawer.set_size(size);
        GLDrawer {
            window: loader.window,
            events: loader.events,
            drawer,
        }
    }

    pub fn update(&mut self) {
        self.drawer.update();
    }

    pub fn start_main_loop(&mut self) {
        self.drawer.init();

        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(key, scancode, action, mods) => {
                        self.window.set_should_close(true)
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.drawer.on_cursor_moved(Point {
                            x: xpos as f32,
                            y: ypos as f32,
                        });
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        self.drawer.set_size(Size {
                            width: width as f32,
                            height: height as f32,
                        });
                    }
                    glfw::WindowEvent::MouseButton(btn, action, mods) => {
                        self.drawer.on_mouse_key_pressed(
                            MouseButton::from_glfw(btn),
                            ButtonState::from_glfw(action),
                        )
                    }
                    _ => {}
                }
            }

            self.update();
            self.window.swap_buffers();
        }
    }
}
