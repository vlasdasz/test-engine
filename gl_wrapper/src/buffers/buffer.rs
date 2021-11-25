use std::ffi::c_void;

#[cfg(any(target_os = "ios", target_os = "android"))]
use gles31_sys::*;
use tools::array_view::ArrayView;

use crate::BufferConfig;

#[derive(Debug)]
pub struct Buffer {
    vertex_data: ArrayView<f32>,
    indices:     Option<ArrayView<u16>>,

    vertices_count: i32,

    vertex_array_object:  u32,
    vertex_buffer_object: u32,
    index_buffer_object:  u32,

    pub draw_mode: u32,
}

impl Buffer {
    pub fn make(
        config: &'static BufferConfig,
        vertex_data: ArrayView<f32>,
        indices: Option<ArrayView<u16>>,
        draw_mode: u32,
    ) -> Buffer {
        let mut vertex_array_object: u32 = u32::MAX;
        let mut vertex_buffer_object: u32 = u32::MAX;
        let mut index_buffer_object: u32 = u32::MAX;

        let vertices_count: i32 = if indices.is_none() {
            vertex_data.size as i32 / config.size() as i32
        } else {
            -1
        };

        GL!(GenVertexArrays, 1, &mut vertex_array_object);
        GL!(BindVertexArray, vertex_array_object);

        GL!(GenBuffers, 1, &mut vertex_buffer_object);
        GL!(BindBuffer, GLC!(ARRAY_BUFFER), vertex_buffer_object);

        cfg_if::cfg_if! {
            if #[cfg(any(target_os="ios", target_os="android"))] {
                cfg_if::cfg_if! {
                    if #[cfg(target_pointer_width = "64")] {
                        type VertexSize = i64;
                    } else {
                        type VertexSize = i32;
                    }
                };
            }
            else {
                type VertexSize = isize;
            }
        };

        GL!(
            BufferData,
            GLC!(ARRAY_BUFFER),
            (vertex_data.size * std::mem::size_of::<GLT!(GLfloat)>()) as VertexSize,
            vertex_data.data as *const c_void,
            GLC!(STATIC_DRAW)
        );

        if let Some(indices) = &indices {
            GL!(GenBuffers, 1, &mut index_buffer_object);
            GL!(BindBuffer, GLC!(ELEMENT_ARRAY_BUFFER), index_buffer_object);
            GL!(
                BufferData,
                GLC!(ELEMENT_ARRAY_BUFFER),
                (indices.size * std::mem::size_of::<GLT!(GLushort)>()) as VertexSize,
                indices.data as *const c_void,
                GLC!(STATIC_DRAW)
            );
        }

        config.set_pointers();
        GL!(BindVertexArray, 0);

        Buffer {
            vertex_data,
            vertices_count,
            indices,
            vertex_array_object,
            vertex_buffer_object,
            index_buffer_object,
            draw_mode,
        }
    }
}

impl Buffer {
    pub fn draw(&self) {
        GL!(BindVertexArray, self.vertex_array_object);

        if let Some(indices) = &self.indices {
            GL!(
                DrawElements,
                self.draw_mode,
                indices.size as i32,
                GLC!(UNSIGNED_SHORT),
                std::ptr::null::<c_void>()
            )
        } else {
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
        //TODO: Check errors
        GL_SILENT!(DeleteBuffers, 1, &self.vertex_buffer_object);
        if self.index_buffer_object != u32::MAX {
            GL_SILENT!(DeleteBuffers, 1, &self.index_buffer_object);
        }
        GL_SILENT!(DeleteVertexArrays, 1, &self.vertex_array_object);
    }
}
