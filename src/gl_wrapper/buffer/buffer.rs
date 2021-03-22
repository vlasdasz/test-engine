use std::ffi::c_void;
use crate::utils::ArrayView;
use crate::gl_wrapper::BufferConfig;

#[derive(Debug)]
pub struct Buffer {
    config: &'static BufferConfig,
    vertex_data: ArrayView<f32>,
    indices: Option<ArrayView<u32>>,
    vertex_array_object:  u32,
    vertex_buffer_object: u32,
    index_buffer_object:  u32
}

impl Buffer {
    pub fn new(config: &'static BufferConfig,
               vertex_data: ArrayView<f32>,
               indices: Option<ArrayView<u32>>
    ) -> Buffer {

        let mut vertex_array_object:  u32 = 0;
        let mut vertex_buffer_object: u32 = 0;
        let mut index_buffer_object:  u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_object);
            gl::BindVertexArray(vertex_array_object);

            gl::GenBuffers(1, &mut vertex_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertex_data.size * std::mem::size_of::<gl::types::GLfloat>()) as isize,
                           vertex_data.data as *const c_void,
                           gl::STATIC_DRAW);

            if let Some(indices) = &indices {
                gl::GenBuffers(1, &mut index_buffer_object);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                               (indices.size * std::mem::size_of::<gl::types::GLushort>()) as isize,
                               indices.data as *const c_void,
                               gl::STATIC_DRAW);
            }

            config.set_pointers();
            gl::BindVertexArray(0);
        }

        Buffer {
            config,
            vertex_data,
            indices,
            vertex_array_object,
            vertex_buffer_object,
            index_buffer_object
        }
    }
}
