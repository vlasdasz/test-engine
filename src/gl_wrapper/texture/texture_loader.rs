#[cfg(target_os = "ios")]
use gles31_sys::*;

use crate::gm::Size;
use std::ffi::c_void;

pub struct TextureLoader;

fn mode_for_channels(channels: u32) -> u32 {
    match channels {
        #[cfg(target_os = "ios")]
        1 => GLC!(LUMINANCE),
        #[cfg(not(target_os = "ios"))]
        1 => GLC!(RED),
        _ => GLC!(RGBA),
    }
}

impl TextureLoader {
    pub fn load(data: *const c_void, size: Size, channels: u32) -> u32 {
        let mut id: u32 = u32::MAX;

        GL!(GenTextures, 1, &mut id);

        GL!(BindTexture, GLC!(TEXTURE_2D), id);

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

        assert_ne!(id, u32::MAX);

        id
    }
}
