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

    pub fn set_clear_color(color: impl Into<Color>) {
        let color = color.into();
        unsafe { STATIC_DATA.clear_color = color };
        GL!(ClearColor, color.r, color.g, color.b, color.a)
    }

    pub fn clear() {
        GL!(
            Clear,
            GLC!(COLOR_BUFFER_BIT) | GLC!(DEPTH_BUFFER_BIT) | GLC!(STENCIL_BUFFER_BIT)
        )
    }

    pub fn clear_with_color(color: impl Into<Color>) {
        let clear_color = Self::clear_color();
        Self::set_clear_color(color);
        Self::clear();
        Self::set_clear_color(clear_color);
    }

    #[allow(clippy::cast_possible_truncation)]
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

    #[allow(clippy::cast_possible_truncation)]
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

    pub fn start_stensil() {
        GL!(Enable, GLC!(STENCIL_TEST));
        GL!(StencilFunc, GLC!(NEVER), 1, 0xFF);
        GL!(StencilOp, GLC!(REPLACE), GLC!(KEEP), GLC!(KEEP));

        GL!(Clear, GLC!(STENCIL_BUFFER_BIT));

        GL!(StencilMask, 0xFF);
        GL!(Clear, GLC!(STENCIL_BUFFER_BIT));
    }

    pub fn draw_stensiled() {
        GL!(StencilMask, 0x00);
        GL!(StencilFunc, GLC!(EQUAL), 1, 0xFF);
    }

    pub fn disable_stensil() {
        GL!(Disable, GLC!(STENCIL_TEST));
    }

    pub fn enable_blend() {
        GL!(Enable, GLC!(BLEND));
        GL!(BlendFunc, GLC!(SRC_ALPHA), GLC!(ONE_MINUS_SRC_ALPHA));
    }

    pub fn enable_depth() {
        GL!(Enable, GLC!(DEPTH_TEST));
        GL!(DepthFunc, GLC!(LEQUAL));
        GL!(DepthRange, 0.0, 1.0);
    }

    pub fn save_default_framebuffer_id() {
        GL!(
            GetIntegerv,
            GLC!(FRAMEBUFFER_BINDING),
            &mut STATIC_DATA.default_framebuffer_id
        );
        trace!("default_framebuffer_id: {}", unsafe {
            STATIC_DATA.default_framebuffer_id
        });
    }

    #[allow(clippy::cast_sign_loss)]
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
