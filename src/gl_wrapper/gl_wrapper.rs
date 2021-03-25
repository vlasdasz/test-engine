
extern crate gl;
extern crate glfw;

use crate::gm::*;

use glfw::{ Action, Context, Key, Glfw, WindowEvent };
use self::glfw::OpenGlProfileHint::Core;
use crate::ui::View;
use self::glfw::Window;

pub struct GLWrapper {
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    window: Window,
    window_size: Size
}

impl GLWrapper {

    pub fn set_clear_color(&self, color: Color) {
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear(&self) {
        GL!(Clear, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
    }

    pub fn enable_depth_test(&self) {
        GL!(Enable, gl::DEPTH_TEST)
    }

    pub fn disable_depth_test(&self) {
        GL!(Disable, gl::DEPTH_TEST)
    }

    pub fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = 2.0;
        GL!(Viewport, (rect.origin.x * SCALE) as i32,
                      ((self.window_size.height - rect.origin.y - rect.size.height) * SCALE) as i32,
                      (rect.size.width * SCALE) as i32,
                      (rect.size.height * SCALE) as i32)
    }

    pub fn start_main_loop(&mut self) {

        self.window.make_current();
        self.window.set_key_polling(true);

        self.set_clear_color(Color::GRAY);

        while !self.window.should_close() {

            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }

            self.clear();
         //   (self.frame)();
            self.window.swap_buffers();
        }
    }

    fn init(&mut self) {

        let mut root_vew = View::new();

        root_vew.set_frame(Rect::from_size(self.window_size));

        root_vew.make_subview(|view| {
            view.set_frame(Rect::make(100.0, 100.0, 200.0, 200.0));
            view.make_subview(|view|{
                view.set_frame(Rect::make(100.0, 100.0, 40.0, 40.0));
                view.make_subview(|view| {
                    view.set_frame(Rect::make(10.0, 10.0, 20.0, 20.0));
                });
            });
        });

    }

    pub fn with_size(size: Size) -> GLWrapper {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Samples(Some(16)));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(Core));

        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }

        let (mut window, events) =
            glfw.create_window(size.width as u32,
                               size.height as u32,
                               "Hello this is window",
                               glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");

        GL!(load_with, |symbol| window.get_proc_address(symbol) as *const _);

        let mut wrapper = GLWrapper { events, glfw, window, window_size: size };
        wrapper.init();

        wrapper
    }

}