
extern crate gl;
extern crate glfw;

use crate::gm::*;
use crate::gm::volume::Vector3;

use crate::te::paths;

use crate::utils::log;
use crate::utils::ArrayView;

use crate::te::paths::PathBufExt;

use super::gl_info::GLInfo;
use crate::gl_wrapper::shader::ShaderCompiler;
use crate::gl_wrapper::buffer::buffer_config::BufferConfig;

use glfw::{Action, Context, Key };
use self::glfw::OpenGlProfileHint::Core;
use crate::gl_wrapper::buffer::buffer::Buffer;

pub struct GL;

impl GL {

    pub fn set_clear_color(color: Color) {
        unsafe { gl::ClearColor(color.r, color.g, color.b, color.a); }
    }

    pub fn clear() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
    }

    pub fn init(size: Size) {

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

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let gl_info = GLInfo::get();

        let shader_compiler = ShaderCompiler { gl_info };

        let _shader = shader_compiler.compile(paths::shaders::sprites().pushing("sprite"));

        log(&shader_compiler.gl_info);

        let original_buffer = vec![
            Vector3 { x: 1.0, y: 2.0, z: 3.0},
            Vector3 { x: 4.0, y: 5.0, z: 6.0},
            Vector3 { x: 7.0, y: 8.0, z: 9.0},
        ];

        let indices = vec![1, 2, 3, 4];

        let buf = Buffer::new(
            &BufferConfig::_3_3_2,
            ArrayView::from_vector(&original_buffer),
            None
        );

        let buf2 = Buffer::new(
            &BufferConfig::_3_3_2,
            ArrayView::from_vector(&original_buffer),
            Some(ArrayView::from_vector(&indices))
        );

        println!("{:?}", buf);
        println!("{:?}", buf2);

        window.make_current();
        window.set_key_polling(true);

        while !window.should_close() {
            window.swap_buffers();

            GL::set_clear_color(Color::random());
            GL::clear();

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
        }
    }

}