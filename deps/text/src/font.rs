use std::{
    ffi::c_void,
    ops::Range,
    path::{Path, PathBuf},
};

use gl_image::Image;
use gm::flat::Size;
use log::trace;
use rtools::{
    data_manager::{DataManager, DataStorage, Handle, LoadFromPath, Managed},
    file::File,
    hash, managed,
};

use crate::glyph::Glyph;

fn render_glyph(font: &fontdue::Font, symbol: char, size: f32) -> Glyph {
    let (metrics, bitmap) = font.rasterize(symbol, size);

    let size = Size {
        width:  metrics.width as f32,
        height: metrics.height as f32,
    };

    let image = Image::from(bitmap.as_ptr() as *const c_void, size, 1, hash(symbol));

    Glyph::new(
        symbol,
        image,
        metrics.advance_width as _,
        (metrics.bounds.xmin, metrics.bounds.height).into(),
    )
}

#[derive(Clone, Debug)]
pub struct Font {
    pub path:           PathBuf,
    pub font:           fontdue::Font,
    pub size:           u32,
    pub height:         f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>,
}

impl Font {
    fn new(path: &Path, size: u32) -> Result<Font, &'static str> {
        trace!("Loading font {:?}", path);

        let data = File::read(path);
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())?;

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

        trace!("Font: OK");

        Ok(Font {
            path: path.into(),
            font,
            size,
            height,
            baseline_shift,
            glyphs,
        })
    }
}

impl Font {
    pub fn default() -> Handle<Self> {
        Font::get("SF.otf")
    }

    pub fn glyph_for_char(&self, ch: char) -> &Glyph {
        debug_assert!(!self.glyphs.is_empty(), "Font is not initialized");
        if ch > 127 as char {
            return &self.glyphs['?' as usize];
        }
        &self.glyphs[ch as usize]
    }
}

impl LoadFromPath for Font {
    fn load(path: &Path) -> Self {
        Font::new(path, 48).unwrap()
    }
}

managed!(Font);
