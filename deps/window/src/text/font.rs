use anyhow::Result;
use gm::LossyConvert;
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, StencilState, TextureFormat};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontRef};

use crate::{RGBA_TEXTURE_FORMAT, window::Window};

pub struct Font {
    pub name:  &'static str,
    pub brush: TextBrush<FontRef<'static>>,
}

impl Font {
    fn new(name: &'static str, data: &'static [u8]) -> Result<Self> {
        let window = Window::current();

        let render_size = Window::render_size();

        let brush = BrushBuilder::using_font_bytes(data)?.with_depth_stencil( DepthStencilState {
            format:              TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare:       CompareFunction::Less,
            stencil:             StencilState::default(),
            bias:                DepthBiasState::default(),
        }.into())
            /* .initial_cache_size((16_384, 16_384))) */ // use this to avoid resizing cache texture
            .build(&window.device, render_size.width.lossy_convert(), render_size.height.lossy_convert(), RGBA_TEXTURE_FORMAT);
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
