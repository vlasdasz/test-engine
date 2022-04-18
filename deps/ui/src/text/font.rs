use std::{ffi::c_void, ops::Range, path::Path};

use gl_image::Image;
use gm::flat::Size;
use rtools::{file::File, misc::hash};

use crate::{Glyph, DEFAULT_FONT};

fn render_glyph(font: &fontdue::Font, symbol: char, size: f32) -> Glyph {
    let (metrics, bitmap) = font.rasterize(symbol, size);

    let size = Size {
        width:  metrics.width as f32,
        height: metrics.height as f32,
    };

    let image = Image::from(bitmap.as_ptr() as *const c_void, size, 1, hash(symbol), None);

    Glyph::new(
        symbol,
        image,
        metrics.advance_width as _,
        (metrics.bounds.xmin, metrics.bounds.height).into(),
    )
}

#[derive(Clone, Debug)]
pub struct Font {
    pub size:           u32,
    pub height:         f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>,
}

impl Font {
    pub fn new(path: &Path, size: u32) -> Result<Font, &'static str> {
        error!("New font {:?}", path);

        let data = File::read(path);
        error!("File::read");
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())?;
        error!("fontdue::Font::from_bytes");

        let mut glyphs = Vec::with_capacity(128);

        let mut y_max = f32::MIN;
        let mut y_min = f32::MAX;

        for symbol in (Range {
            start: 0 as char,
            end:   127 as char,
        }) {
            let glyph = render_glyph(&font, symbol, size as f32);
            if y_max < glyph.y_max() {
                y_max = glyph.y_max()
            }
            if y_min > glyph.y_min() {
                y_min = glyph.y_min()
            }

            glyphs.push(glyph);
        }

        let height = y_max - y_min;
        let baseline_position = y_min.abs();
        let baseline_shift = height / 2.0 - baseline_position;

        error!("Font OK");

        Ok(Font {
            size,
            height,
            baseline_shift,
            glyphs,
        })
    }
}

impl Font {
    pub fn glyph_for_char(&self, ch: char) -> &Glyph {
        debug_assert!(!self.glyphs.is_empty(), "Font is not initialized");
        &self.glyphs[ch as usize]
    }
}

impl Default for Font {
    fn default() -> Self {
        DEFAULT_FONT.lock().unwrap().clone()
    }
}
