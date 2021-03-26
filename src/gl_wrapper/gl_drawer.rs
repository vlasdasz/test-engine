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

        GLWrapper::set_clear_color(&Color::GRAY);

        self.drawer.init();

        while !self.loader.window.should_close() {

            self.loader.window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.loader.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.loader.window.set_should_close(true)
                    },
                    _ => {},
                }
            }

            GLWrapper::clear();
            let win_size = self.loader.window.get_size();

            self.loader.window_size = Size { width: win_size.0 as f32, height: win_size.1 as f32 };
            self.drawer.update(&self.loader.window_size);
            self.loader.window.swap_buffers();
        }
    }

}