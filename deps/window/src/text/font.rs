use anyhow::Result;
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontRef};

use crate::{utils::depth_stencil_state, window::Window};

pub struct Font {
    pub name:  &'static str,
    pub brush: TextBrush<FontRef<'static>>,
}

impl Font {
    fn new(name: &'static str, data: &'static [u8]) -> Result<Self> {
        let app = Window::current();
        let brush = BrushBuilder::using_font_bytes(data)?.with_depth_stencil(depth_stencil_state().into())
            /* .initial_cache_size((16_384, 16_384))) */ // use this to avoid resizing cache texture
            .build(&app.device, app.config.width, app.config.height, app.config.format);
        Ok(Self { name, brush })
    }
}

impl Font {
    pub fn helvetice() -> &'static mut Self {
        let state = &mut Window::current().state;
        state
            .fonts
            .entry("Helvetica.ttf")
            .or_insert_with(|| Self::new("Helvetica.ttf", include_bytes!("fonts/Helvetica.ttf")).unwrap())
    }

    pub fn with_name(name: &'static str) -> &'static mut Self {
        Window::current().state.fonts.get_mut(name).unwrap()
    }
}
