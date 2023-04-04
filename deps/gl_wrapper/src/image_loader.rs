use std::ffi::c_void;

#[cfg(mobile)]
use gles31_sys::*;
use gm::flat::Size;

use crate::buffers::FrameBuffer;

pub struct ImageLoader;

fn mode_for_channels(channels: u32) -> u32 {
    match channels {
        #[cfg(mobile)]
        1 => GLC!(LUMINANCE),
        #[cfg(desktop)]
        1 => GLC!(RED),
        _ => GLC!(RGBA),
    }
}

impl ImageLoader {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn load(data: *const c_void, size: Size, channels: u32) -> FrameBuffer {
        let mut texture_handle = u32::MAX;

        GL!(GenTextures, 1, &mut texture_handle);

        GL!(BindTexture, GLC!(TEXTURE_2D), texture_handle);

        if channels == 1 {
            GL!(PixelStorei, GLC!(UNPACK_ALIGNMENT), 1);
        }

        GL!(
            TexImage2D,
            GLC!(TEXTURE_2D),
            0,
            mode_for_channels(channels) as i32,
            size.width as i32,
            size.height as i32,
            0,
            mode_for_channels(channels),
            GLC!(UNSIGNED_BYTE),
            data
        );

        GL!(GenerateMipmap, GLC!(TEXTURE_2D));
        GL!(
            TexParameterf,
            GLC!(TEXTURE_2D),
            GLC!(TEXTURE_MIN_FILTER),
            GLC!(LINEAR) as f32
        );
        GL!(
            TexParameterf,
            GLC!(TEXTURE_2D),
            GLC!(TEXTURE_MAG_FILTER),
            GLC!(LINEAR) as f32
        );

        assert_ne!(texture_handle, u32::MAX);

        FrameBuffer {
            texture_handle,
            buffer_handle: u32::MAX,
        }
    }
}
