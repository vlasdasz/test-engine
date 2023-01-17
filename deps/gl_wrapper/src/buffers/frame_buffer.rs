use std::ptr::null;

#[cfg(mobile)]
use gles31_sys::*;
use gm::{flat::Size, Color};

use crate::GLWrapper;

#[derive(Debug)]
pub struct FrameBuffer {
    pub buffer_handle:  u32,
    pub texture_handle: u32,
}

impl FrameBuffer {
    pub fn bind(&self) {
        GL!(BindFramebuffer, GLC!(FRAMEBUFFER), self.buffer_handle);
    }

    pub fn unbind(&self) {
        GLWrapper::unbind_framebuffer();
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        if self.buffer_handle != u32::MAX {
            GL!(DeleteFramebuffers, 1, &self.buffer_handle);
        }
        if self.texture_handle != u32::MAX {
            GL!(DeleteTextures, 1, &self.texture_handle);
        }
    }
}

impl<T: Into<Size>> From<T> for FrameBuffer {
    fn from(size: T) -> Self {
        let size = size.into();

        debug_assert!(size.is_valid(), "Invalid framebuffer size: {size:?}");

        let mut buffer_handle = u32::MAX;

        GL!(GenFramebuffers, 1, &mut buffer_handle);
        GL!(BindFramebuffer, GLC!(FRAMEBUFFER), buffer_handle);

        let mut texture_handle = u32::MAX;
        GL!(GenTextures, 1, &mut texture_handle);

        GL!(BindTexture, GLC!(TEXTURE_2D), texture_handle);
        GL!(
            TexImage2D,
            GLC!(TEXTURE_2D),
            0,
            GLC!(RGB) as _,
            size.width as _,
            size.height as _,
            0,
            GLC!(RGB),
            GLC!(UNSIGNED_BYTE),
            null()
        );

        GL!(
            TexParameteri,
            GLC!(TEXTURE_2D),
            GLC!(TEXTURE_MAG_FILTER),
            GLC!(NEAREST) as _
        );

        GL!(
            TexParameteri,
            GLC!(TEXTURE_2D),
            GLC!(TEXTURE_MIN_FILTER),
            GLC!(NEAREST) as _
        );

        GL!(
            FramebufferTexture2D,
            GLC!(FRAMEBUFFER),
            GLC!(COLOR_ATTACHMENT0),
            GLC!(TEXTURE_2D),
            texture_handle,
            0
        );

        GL!(DrawBuffers, 1, &GLC!(COLOR_ATTACHMENT0));

        if GL!(CheckFramebufferStatus, GLC!(FRAMEBUFFER)) != GLC!(FRAMEBUFFER_COMPLETE) {
            panic!("Failed to initialize framebuffer")
        }

        GLWrapper::clear_with_color(Color::CLEAR);
        GLWrapper::unbind_framebuffer();

        Self {
            buffer_handle,
            texture_handle,
        }
    }
}
