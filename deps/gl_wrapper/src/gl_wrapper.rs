#![allow(clippy::mismatched_target_os)]

use std::borrow::Borrow;

#[cfg(mobile)]
use gles31_sys::*;
use gm::{flat::Rect, Color};

pub struct GLWrapper;

impl GLWrapper {
    pub fn bind_texture(id: u32) {
        debug_assert!(id != u32::MAX, "Invalid texture handle");
        GL!(BindTexture, GLC!(TEXTURE_2D), id);
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

    pub fn set_ui_viewport(window_height: f32, scale: f32, rect: impl Into<Rect>) {
        let rect = rect.into();
        if rect.size.is_invalid() {
            return;
        }
        let scale = adjust_scale(scale);

        Self::set_viewport((
            rect.origin.x * scale,
            (window_height - rect.origin.y - rect.size.height) * scale,
            rect.size.width * scale,
            rect.size.height * scale,
        ));
    }

    pub fn set_viewport(rect: impl Into<Rect>) {
        let rect = rect.into();
        GL!(
            Viewport,
            rect.x() as _,
            rect.y() as _,
            rect.width() as _,
            rect.height() as _
        );
    }

    pub fn scissor(rect: impl Into<Rect>, mut draw: impl FnMut()) {
        let rect = rect.into();
        GL!(Enable, GLC!(SCISSOR_TEST));
        GL!(
            Scissor,
            rect.x() as _,
            rect.y() as _,
            rect.width() as _,
            rect.height() as _
        );
        draw();
        GL!(Disable, GLC!(SCISSOR_TEST));
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
