use std::{ops::Range, path::Path, sync::Mutex};

use gl_image::Image;
use gm::flat::Size;
use log::trace;
use rtools::{
    data_manager::{DataManager, Handle, ResourceLoader},
    file::File,
    hash, managed,
};

use crate::glyph::Glyph;

static RENDER: Mutex<bool> = Mutex::new(true);

pub static DEFAULT_FONT_SIZE: f32 = 64.0;

fn render_glyph(font: &fontdue::Font, symbol: char, size: f32) -> Glyph {
    let (metrics, bitmap) = font.rasterize(symbol, size);

    let size = Size {
        width:  metrics.width as f32,
        height: metrics.height as f32,
    };

    let image = Image::from(
        bitmap.as_ptr().cast(),
        size,
        1,
        hash(symbol),
        format!("Glyph: {symbol}"),
    );

    Glyph::new(
        symbol,
        image,
        metrics.advance_width as _,
        (metrics.bounds.xmin, metrics.bounds.height).into(),
    )
}

#[derive(Clone, Debug)]
pub struct Font {
    pub name:           String,
    pub font:           fontdue::Font,
    pub size:           f32,
    pub height:         f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>,
}

impl Font {
    fn from_data(name: impl ToString, data: &[u8], size: f32) -> Result<Font, &'static str> {
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())?;

        let mut glyphs = Vec::with_capacity(128);

        let mut y_max = f32::MIN;
        let mut y_min = f32::MAX;

        if Font::render_enabled() {
            for symbol in (Range {
                start: 0 as char,
                end:   127 as char,
            }) {
                let glyph = render_glyph(&font, symbol, size);
                if y_max < glyph.y_max() {
                    y_max = glyph.y_max()
                }
                if y_min > glyph.y_min() {
                    y_min = glyph.y_min()
                }

                glyphs.push(glyph);
            }
        }

        let height = y_max - y_min;
        let baseline_position = y_min.abs();
        let baseline_shift = height / 2.0 - baseline_position;

        trace!("Font: OK");

        Ok(Font {
            name: name.to_string(),
            font,
            size,
            height,
            baseline_shift,
            glyphs,
        })
    }

    fn load_path(path: &Path, size: f32) -> Result<Font, &'static str> {
        Self::load_data(&File::read(path), size, path.display())
    }

    fn load_data(data: &[u8], size: f32, name: impl ToString) -> Result<Font, &'static str> {
        trace!("Loading font {:?}", name.to_string());
        Self::from_data(name, data, size)
    }
}

impl Font {
    pub fn san_francisco() -> Handle<Self> {
        const SF: &str = "default_sf";

        if let Some(sf) = Font::handle_with_name(SF) {
            return sf;
        }

        let sf = Font::from_data(SF, include_bytes!("fonts/SF.otf"), DEFAULT_FONT_SIZE)
            .expect("BUG: Failed to render default font");

        Font::add_with_name(SF, sf)
    }

    pub fn disable_render() {
        *RENDER.lock().unwrap() = false
    }

    #[cfg(debug_assertions)]
    pub fn render_enabled() -> bool {
        *RENDER.lock().unwrap()
    }

    #[cfg(not(debug_assertions))]
    pub fn render_enabled() -> bool {
        true
    }

    pub fn glyph_for_char(&self, ch: char) -> &Glyph {
        debug_assert!(!self.glyphs.is_empty(), "Font is not initialized");
        if ch > 127 as char {
            return &self.glyphs['?' as usize];
        }
        &self.glyphs[ch as usize]
    }
}

impl ResourceLoader for Font {
    fn load_path(path: &Path) -> Self {
        Font::load_path(path, DEFAULT_FONT_SIZE).unwrap()
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        Font::load_data(data, DEFAULT_FONT_SIZE, name).unwrap()
    }
}

managed!(Font);
