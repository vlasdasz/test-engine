use crate::gl_wrapper::{GLLoader, GLWrapper};
use crate::gm::{Color, Size, Point};

use glfw::{Action, Context, Window, WindowEvent};
use crate::gl_wrapper::gl_drawer::MouseButton::Undefined;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Undefined
}

impl MouseButton {
    pub fn from_gl(btn: glfw::MouseButton) -> Self {
        match btn {
            glfw::MouseButtonLeft   => Self::Left,
            glfw::MouseButtonRight  => Self::Right,
            glfw::MouseButtonMiddle => Self::Middle,
            _ => Undefined
        }
    }
}

pub enum ButtonState {
    Up,
    Down,
    Repeat
}

impl ButtonState {
    pub fn from_gl(action: Action) -> Self {
        match action {
            Action::Release => Self::Up,
            Action::Press   => Self::Down,
            Action::Repeat  => Self::Repeat
        }
    }
}

pub trait Updatable {
    fn new() -> Self;
    fn init(&mut self);
    fn set_size(&mut self, size: Size);
    fn on_cursor_moved(&mut self, position: Point);
    fn on_mouse_key_pressed(&mut self, button: MouseButton, state: ButtonState);
    fn update(&mut self);
}

pub struct GLDrawer<T: Updatable> {
    window: Window,
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
    drawer: T
}

impl<T: Updatable> GLDrawer<T> {

    pub fn with_size(size: Size) -> GLDrawer<T> {
        let loader = GLLoader::with_size(size);
        let mut drawer = T::new();
        drawer.set_size(size);
        GLDrawer {
            window: loader.window,
            events: loader.events,
            drawer
        }
    }

    pub fn start_main_loop(&mut self) {

        GLWrapper::set_clear_color(&Color::GRAY);

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
                    },
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.drawer.on_cursor_moved(Point { x: xpos as f32, y: ypos as f32 });
                    },
                    glfw::WindowEvent::Size(width, height) => {
                        self.drawer.set_size(Size { width: width as f32, height: height as f32 });
                    },
                    glfw::WindowEvent::MouseButton(btn, action, mods) => {
                        self.drawer.on_mouse_key_pressed(
                            MouseButton::from_gl(btn),
                            ButtonState::from_gl(action)
                        )
                    },
                    _ => {},
                }
            }

            GLWrapper::clear();

            self.drawer.update();
            self.window.swap_buffers();
        }
    }

}