#![allow(clippy::mismatched_target_os)]

use std::borrow::Borrow;

#[cfg(mobile)]
use gles31_sys::*;
use gm::{flat::Rect, Color};
use rtools::platform::Platform;

pub struct GLWrapper;

struct StaticData {
    default_framebuffer_id: i32,
    clear_color:            Color,
}

static mut STATIC_DATA: StaticData = StaticData {
    default_framebuffer_id: -1,
    clear_color:            Color::CLEAR,
};

impl GLWrapper {
    pub fn bind_texture(id: u32) {
        debug_assert!(id != u32::MAX, "Invalid texture handle");
        GL!(BindTexture, GLC!(TEXTURE_2D), id);
    }

    pub fn clear_color() -> Color {
        unsafe { STATIC_DATA.clear_color }
    }

    pub fn set_clear_color(color: impl Borrow<Color>) {
        let color = color.borrow();
        unsafe { STATIC_DATA.clear_color = *color };
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear() {
        GL!(
            Clear,
            GLC!(COLOR_BUFFER_BIT) | GLC!(DEPTH_BUFFER_BIT) | GLC!(STENCIL_BUFFER_BIT)
        )
    }

    pub fn set_ui_viewport(window_height: f32, scale: f32, rect: impl Into<Rect>) {
        let rect = rect.into();
        if rect.size.is_invalid() {
            dbg!("invalid viewport");
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

    pub fn save_default_framebuffer_id() {
        GL!(
            GetIntegerv,
            GLC!(FRAMEBUFFER_BINDING),
            &mut STATIC_DATA.default_framebuffer_id
        );
        dbg!(unsafe { STATIC_DATA.default_framebuffer_id as u32 });
    }

    fn default_framebuffer_id() -> u32 {
        if Platform::IOS {
            unsafe { STATIC_DATA.default_framebuffer_id as u32 }
        } else {
            0
        }
    }

    pub fn unbind_framebuffer() {
        GL!(BindFramebuffer, GLC!(FRAMEBUFFER), Self::default_framebuffer_id());
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
