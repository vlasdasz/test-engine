#[cfg(any(target_os = "ios", target_os = "android"))]
use gles31_sys::*;
use gm::{Color, Rect};
use tools::*;

pub struct GLWrapper;

impl GLWrapper {
    pub fn bind_image(id: u32) {
        if id == u32::MAX {
            panic!("Invalid image gl_handle");
        }
        GL!(BindTexture, GLC!(TEXTURE_2D), id);
    }

    pub fn set_clear_color(color: &Color) {
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear() {
        GL!(Clear, GLC!(COLOR_BUFFER_BIT) | GLC!(DEPTH_BUFFER_BIT))
    }

    pub fn enable_depth_test() {
        GL!(Enable, GLC!(DEPTH_TEST))
    }

    pub fn disable_depth_test() {
        GL!(Disable, GLC!(DEPTH_TEST))
    }

    pub fn set_viewport(window_height: f32, scale: &'static f32, rect: &Rect) {
        GL!(
            Viewport,
            (rect.origin.x * scale) as i32,
            ((window_height - rect.origin.y - rect.size.height) * scale) as i32,
            (rect.size.width * scale) as i32,
            (rect.size.height * scale) as i32
        )
    }

    pub fn enable_blend() {
        GL!(Enable, GLC!(BLEND));
        GL!(BlendFunc, GLC!(SRC_ALPHA), GLC!(ONE_MINUS_SRC_ALPHA));
    }
}
