use crate::gl_wrapper::GLLoader;
use crate::gm::{Point, Size};
use crate::sprites::Level;
use crate::te::Screen;
use crate::tools::New;
use crate::ui::input::touch::{ButtonState, MouseButton};
use crate::ui::view::View;
use glfw::{Context, Window, WindowEvent};
use tools::refs::Shared;

pub struct GLDrawer {
    window: Window,
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
    screen: Screen,
    size: Size,
}

impl GLDrawer {
    pub fn with_size(size: Size) -> GLDrawer {
        let loader = GLLoader::with_size(size);
        GLDrawer {
            window: loader.window,
            events: loader.events,
            screen: Screen::new(),
            size,
        }
    }

    pub fn update(&mut self) {
        self.screen.update();
    }

    pub fn start_main_loop(&mut self) {
        self.screen.init();

        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        self.screen.set_size(self.size);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(key, _, _, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.screen.on_cursor_moved(Point {
                            x: xpos as f32,
                            y: ypos as f32,
                        });
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        self.screen.set_size(Size {
                            width: width as f32,
                            height: height as f32,
                        });
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        self.screen.on_mouse_key_pressed(
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
