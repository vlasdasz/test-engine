use std::ffi::c_void;
use crate::gm::Size;

pub struct TextureLoader;

fn mode_for_channels(channels: u32) -> u32 {
    match channels {
        1 => gl::RED,
        _ => gl::RGBA
    }
}

impl TextureLoader {

    pub fn load(data: *const c_void, size: Size, channels: u32) -> u32 {
        let mut id: u32 = u32::MAX;

        GL!(GenTextures, 1, &mut id);

        GL!(BindTexture, gl::TEXTURE_2D, id);

        if channels == 1 {
            GL!(PixelStorei, gl::UNPACK_ALIGNMENT, 1);
        }

        log!(size);

        GL!(TexImage2D,
            gl::TEXTURE_2D,
            0,
            mode_for_channels(channels) as i32,
            size.width as i32,
            size.height as i32,
            0,
            mode_for_channels(channels),
            gl::UNSIGNED_BYTE,
            data);

        GL!(GenerateMipmap, gl::TEXTURE_2D);
        GL!(TexParameterf, gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
        GL!(TexParameterf, gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);

        assert_ne!(id, u32::MAX);

        id
    }
}