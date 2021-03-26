use crate::gl_wrapper::{GLLoader, Updatable, GLWrapper};
use crate::gm::{Color, Size};

use glfw::{ Action, Context, Key };

pub struct GLDrawer<T: Updatable> {
    loader: GLLoader,
    drawer: T
}

impl<T: Updatable> GLDrawer<T> {

    pub fn with_size(size: Size) -> GLDrawer<T> {
        GLDrawer { loader: GLLoader::with_size(size), drawer: T::new() }
    }

    pub fn start_main_loop(&mut self) {

        self.drawer.set_size(self.loader.window_size);

        GLWrapper::set_clear_color(&Color::GRAY);

        self.drawer.init();

        self.loader.window.set_key_polling(true);
        self.loader.window.set_size_polling(true);
        self.loader.window.set_cursor_pos_polling(true);
        self.loader.window.set_mouse_button_polling(true);

        while !self.loader.window.should_close() {

            self.loader.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.loader.events) {
                match event {
                    glfw::WindowEvent::Key(key, scancode, action, mods) => {
                        self.loader.window.set_should_close(true)
                    },
                    glfw::WindowEvent::CursorPos(xpos, ypos) => self.loader.window.set_title(
                        &format!("Cursor position: ({:?}, {:?})", xpos, ypos)
                    ),
                    glfw::WindowEvent::Size(width, height) => {
                        self.drawer.set_size(Size { width: width as f32, height: height as f32 });
                    },
                    _ => {},
                }
            }

            GLWrapper::clear();

            self.drawer.update();
            self.loader.window.swap_buffers();
        }
    }

}