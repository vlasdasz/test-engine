use std::{ffi::c_void, mem::size_of_val, ptr::null};

#[cfg(mobile)]
use gles31_sys::*;

use crate::BufferConfig;

#[derive(Debug)]
pub struct Buffer {
    vertex_data: &'static [f32],
    indices:     Option<&'static [u16]>,

    vertices_count: i32,

    vertex_array_object:  u32,
    vertex_buffer_object: u32,
    index_buffer_object:  u32,

    pub draw_mode: u32,
}

impl Buffer {
    #[allow(clippy::cast_possible_wrap)]
    pub fn make(
        config: &'static BufferConfig,
        vertex_data: &'static [f32],
        indices: Option<&'static [u16]>,
        draw_mode: u32,
    ) -> Buffer {
        let mut vertex_array_object: u32 = u32::MAX;
        let mut vertex_buffer_object: u32 = u32::MAX;
        let mut index_buffer_object: u32 = u32::MAX;

        let vertices_count: i32 = if indices.is_none() {
            i32::try_from(vertex_data.len()).unwrap() / i32::from(config.size())
        } else {
            -1
        };

        GL!(GenVertexArrays, 1, &mut vertex_array_object);
        GL!(BindVertexArray, vertex_array_object);

        GL!(GenBuffers, 1, &mut vertex_buffer_object);
        GL!(BindBuffer, GLC!(ARRAY_BUFFER), vertex_buffer_object);

        GL!(
            BufferData,
            GLC!(ARRAY_BUFFER),
            size_of_val(vertex_data) as _,
            vertex_data.as_ptr().cast::<c_void>(),
            GLC!(STATIC_DRAW)
        );

        if let Some(indices) = indices {
            GL!(GenBuffers, 1, &mut index_buffer_object);
            GL!(BindBuffer, GLC!(ELEMENT_ARRAY_BUFFER), index_buffer_object);
            GL!(
                BufferData,
                GLC!(ELEMENT_ARRAY_BUFFER),
                size_of_val(indices) as _,
                indices.as_ptr().cast::<c_void>(),
                GLC!(STATIC_DRAW)
            );
        }

        config.set_pointers();
        GL!(BindVertexArray, 0);

        Buffer {
            vertex_data,
            indices,
            vertices_count,
            vertex_array_object,
            vertex_buffer_object,
            index_buffer_object,
            draw_mode,
        }
    }
}

impl Buffer {
    pub fn draw_with_mode(&self, draw_mode: u32) {
        GL!(BindVertexArray, self.vertex_array_object);

        if let Some(indices) = &self.indices {
            GL!(
                DrawElements,
                draw_mode,
                indices.len().try_into().unwrap(),
                GLC!(UNSIGNED_SHORT),
                null()
            )
        } else {
            GL!(DrawArrays, draw_mode, 0, self.vertices_count)
        }

        GL!(BindVertexArray, 0);
    }

    pub fn draw(&self) {
        self.draw_with_mode(self.draw_mode)
    }

    pub fn print(&self) {
        if let Some(indices) = &self.indices {
            dbg!(indices);
        }
        dbg!(self.vertex_data);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        //TODO: Check errors
        GL_SILENT!(DeleteBuffers, 1, &self.vertex_buffer_object);
        if self.index_buffer_object != u32::MAX {
            GL_SILENT!(DeleteBuffers, 1, &self.index_buffer_object);
        }
        GL_SILENT!(DeleteVertexArrays, 1, &self.vertex_array_object);
    }
}
