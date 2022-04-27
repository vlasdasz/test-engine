#![allow(clippy::mismatched_target_os)]

use std::borrow::Borrow;

#[cfg(mobile)]
use gles31_sys::*;
use gm::{flat::Rect, Color};

pub struct GLWrapper;

impl GLWrapper {
    pub fn bind_texture(id: u32) {
        debug_assert!(id != u32::MAX, "Invalid texture handle");
        GL!(BindTexture, GLC!(TEXTURE_2D), 133);
    }

    pub fn set_clear_color(color: impl Borrow<Color>) {
        let color = color.borrow();
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

    pub fn set_viewport(window_height: f32, scale: f32, rect: impl Into<Rect>) {
        let rect = rect.into();
        if rect.size.is_invalid() {
            return;
        }
        let scale = adjust_scale(scale);
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

#[cfg(android)]
fn adjust_scale(_scale: f32) -> f32 {
    1.0
}

#[cfg(not(android))]
fn adjust_scale(scale: f32) -> f32 {
    scale
}
