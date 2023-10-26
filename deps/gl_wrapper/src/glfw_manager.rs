use std::{process::ExitCode, sync::atomic::Ordering};

use glfw::{Context, Window};
use gm::flat::IntSize;

use crate::{gl_loader::GLFWEvents, monitor::Monitor, system_events::SystemEvents, GLLoader};

pub struct GLFWManager {
    window:       Window,
    gl_events:    GLFWEvents,
    pub monitors: Vec<Monitor>,
}

impl GLFWManager {
    pub fn new(size: IntSize) -> Self {
        let mut loader = GLLoader::new(size);
        let monitors = loader.monitors();
        Self {
            window: loader.window,
            gl_events: loader.events,
            monitors,
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn take_screenshot(&self, path: impl ToString) {
        let (width, height) = self.window.get_framebuffer_size();

        dbg!(width);
        dbg!(height);

        dbg!(path.to_string());

        todo!()
    }

    pub fn start_main_loop(&mut self) -> ExitCode {
        self.window.set_key_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);
        self.window.set_scroll_polling(true);
        self.window.set_drag_and_drop_polling(true);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            let events = SystemEvents::get();

            let terminate = events.terminate.load(Ordering::Relaxed);

            if terminate != i32::MIN {
                return u8::try_from(terminate).unwrap().into();
            }

            for (_, event) in glfw::flush_messages(&self.gl_events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                        events.key_pressed.trigger((key, action))
                    }
                    glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                        events.cursor_moved.trigger((x_pos, y_pos).into())
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        events.mouse_click.trigger((btn, action))
                    }
                    glfw::WindowEvent::Scroll(x, y) => events.scroll.trigger((x, y).into()),
                    glfw::WindowEvent::FileDrop(paths) => events.file_drop.trigger(paths),
                    glfw::WindowEvent::Close => return ExitCode::SUCCESS,
                    _ => {}
                }
            }

            events.frame_drawn.trigger(());
        }

        ExitCode::SUCCESS
    }

    pub fn set_size(&mut self, size: IntSize) {
        #[allow(clippy::cast_possible_truncation)]
        self.window
            .set_size(size.width.try_into().unwrap(), size.height.try_into().unwrap())
    }
}
