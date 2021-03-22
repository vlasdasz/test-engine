use std::ffi::c_void;
use crate::utils::ArrayView;
use crate::gl_wrapper::BufferConfig;

#[derive(Debug)]
pub struct Buffer {
    config: &'static BufferConfig,

    vertex_data: ArrayView<f32>,
    indices: Option<ArrayView<u32>>,

    vertices_count: i32,

    vertex_array_object:  u32,
    vertex_buffer_object: u32,
    index_buffer_object:  u32,

    pub draw_mode: u32
}

impl Buffer {
    pub fn new(config: &'static BufferConfig,
               vertex_data: ArrayView<f32>,
               indices: Option<ArrayView<u32>>,
               draw_mode: u32
    ) -> Buffer {

        let mut vertex_array_object:  u32 = u32::MAX;
        let mut vertex_buffer_object: u32 = u32::MAX;
        let mut index_buffer_object:  u32 = u32::MAX;

        let vertices_count: i32 =
            if indices.is_none() { vertex_data.size as i32 / config.size() as i32 }
            else { -1 };

        GL!(GenVertexArrays, 1, &mut vertex_array_object);
        GL!(BindVertexArray, vertex_array_object);

        GL!(GenBuffers, 1, &mut vertex_buffer_object);
        GL!(BindBuffer, gl::ARRAY_BUFFER, vertex_buffer_object);

        GL!(BufferData, gl::ARRAY_BUFFER,
                           (vertex_data.size * std::mem::size_of::<gl::types::GLfloat>()) as isize,
                           vertex_data.data as *const c_void,
                           gl::STATIC_DRAW);

        if let Some(indices) = &indices {
            GL!(GenBuffers, 1, &mut index_buffer_object);
            GL!(BindBuffer, gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);
            GL!(BufferData, gl::ELEMENT_ARRAY_BUFFER,
                           (indices.size * std::mem::size_of::<gl::types::GLushort>()) as isize,
                           indices.data as *const c_void,
                           gl::STATIC_DRAW);
        }

        config.set_pointers();
        GL!(BindVertexArray, 0);

        Buffer {
            config,
            vertex_data,
            vertices_count,
            indices,
            vertex_array_object,
            vertex_buffer_object,
            index_buffer_object,
            draw_mode
        }
    }
}

impl Buffer {
    pub fn draw(&self) {
        GL!(BindVertexArray, self.vertex_array_object);

        if let Some(indices) = &self.indices {
            GL!(DrawElements, self.draw_mode,
                              indices.size as i32,
                              gl::UNSIGNED_SHORT,
                              0 as *const c_void)
        }
        else {
            GL!(DrawArrays, self.draw_mode, 0, self.vertices_count)
        }

        GL!(BindVertexArray, 0);
    }

    pub fn print(&self) {
        if let Some(indices) = &self.indices {
            indices.print();
        }
        self.vertex_data.print();
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        GL!(DeleteBuffers, 1, &self.vertex_buffer_object);
        if self.index_buffer_object != u32::MAX {
            GL!(DeleteBuffers, 1, &self.index_buffer_object);
        }
        GL!(DeleteVertexArrays, 1, &self.vertex_array_object);
    }
}