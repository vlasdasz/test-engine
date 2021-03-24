
extern crate gl;
extern crate glfw;

use crate::gm::*;

use crate::te::{Assets, TEUIDrawer};

use glfw::{ Action, Context, Key };
use self::glfw::OpenGlProfileHint::Core;
use crate::ui::View;

pub struct GLWrapper {
    window_size: Size
}

impl GLWrapper {

    pub fn set_clear_color(color: Color) {
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear() {
        GL!(Clear, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
    }

    pub fn enable_depth_test() {
        GL!(Enable, gl::DEPTH_TEST)
    }

    pub fn disable_depth_test() {
        GL!(Disable, gl::DEPTH_TEST)
    }

    pub fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = 2.0;
        GL!(Viewport, (rect.origin.x * SCALE) as i32,
                      ((self.window_size.height - rect.origin.y - rect.size.height) * SCALE) as i32,
                      (rect.size.width * SCALE) as i32,
                      (rect.size.height * SCALE) as i32)
    }

    pub fn init(size: Size) {

        log!(size);

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

        let assets = Assets::init();
        let gl_wrapper = GLWrapper { window_size: size };

        let ui_drawer = TEUIDrawer::new(&gl_wrapper, &assets);

        let mut root_vew = View::new();

        root_vew.set_frame(Rect::from_size(size));

        root_vew.make_subview(|view| {
            view.set_frame(Rect::make(100.0, 100.0, 200.0, 200.0));
            view.make_subview(|view|{
                view.set_frame(Rect::make(100.0, 100.0, 40.0, 40.0));
                view.make_subview(|view| {
                    view.set_frame(Rect::make(10.0, 10.0, 20.0, 20.0));
                });
            });
        });

        log!(root_vew);

        window.make_current();
        window.set_key_polling(true);

        GLWrapper::set_clear_color(Color::GRAY);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                    _ => {},
                }
            }

            GLWrapper::disable_depth_test();

            GLWrapper::clear();

            ui_drawer.draw_view(&mut root_vew);

            window.swap_buffers();
        }
    }

}