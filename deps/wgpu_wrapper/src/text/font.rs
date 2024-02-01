use anyhow::Result;
use wgpu_text::{glyph_brush::ab_glyph::FontRef, BrushBuilder, TextBrush};

use crate::app::App;

pub struct Font {
    pub name:         &'static str,
    pub(crate) brush: TextBrush<FontRef<'static>>,
}

impl Font {
    fn new(name: &'static str, data: &'static [u8]) -> Result<Self> {
        let state = &App::current().state;
        let brush = BrushBuilder::using_font_bytes(data)?
            /* .initial_cache_size((16_384, 16_384))) */ // use this to avoid resizing cache texture
            .build(&state.device, state.config.width, state.config.height, state.config.format);
        Ok(Self { name, brush })
    }
}

impl Font {
    pub fn helvetice() -> &'static mut Self {
        let state = &mut App::current().state;
        state
            .fonts
            .entry("Helvetica.ttf")
            .or_insert_with(|| Self::new("Helvetica.ttf", include_bytes!("fonts/Helvetica.ttf")).unwrap())
    }

    pub fn with_name(name: &'static str) -> &'static mut Self {
        App::current().state.fonts.get_mut(name).unwrap()
    }
}
