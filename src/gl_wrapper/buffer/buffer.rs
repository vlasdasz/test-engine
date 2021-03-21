use crate::gl_wrapper::buffer::buffer_config::BufferConfig;

#[derive(Debug)]
pub struct Buffer {
    pub config: &'static BufferConfig
}