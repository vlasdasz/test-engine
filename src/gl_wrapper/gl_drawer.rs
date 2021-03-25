use crate::gl_wrapper::{GLLoader, Updatable};
use crate::gm::Color;

use glfw::{ Action, Context, Key, Glfw, WindowEvent, OpenGlProfileHint::Core, Window };

pub struct GLDrawer<'a,
    T: Updatable + 'a
> {
    loader: &'a mut GLLoader,
    drawer: T
}

impl<'a,
    T: Updatable
> GLDrawer<'a,
    T
> {

    pub fn new(loader: &'a mut GLLoader,
               drawer: T
    ) -> GLDrawer<'a,
        T
    > {
        GLDrawer { loader,
            drawer
        }
    }

    pub fn start_main_loop(&mut self) {

        self.loader.set_clear_color(Color::GRAY);

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

            self.loader.clear();
         //   self.drawer.update();
            self.loader.window.swap_buffers();
        }
    }

}