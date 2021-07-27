use crate::Glyph;
use gl_image::Image;
use gm::Size;
use std::ffi::c_void;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use tools::new;

fn render_glyph(font: &fontdue::Font, symbol: char, size: f32) -> Glyph {
    let (metrics, bitmap) = font.rasterize(symbol, size);

    let size = Size {
        width: metrics.width as f32,
        height: metrics.height as f32,
    };

    let image = Image::from(bitmap.as_ptr() as *const c_void, size, 1);

    Glyph::new(symbol, image, 20, new())
}

#[derive(Debug, Clone)]
pub struct Font {
    pub size: u32,
    pub height: f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>,
}

impl Font {
    pub fn invalid() -> Self {
        Font {
            size: 0,
            height: 0.0,
            baseline_shift: 0.0,
            glyphs: vec![],
        }
    }

    pub fn new(path: &PathBuf, size: u32) -> Result<Font, &'static str> {
        let data = fs::read(path).unwrap();
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())?;

        let mut glyphs = Vec::with_capacity(128);

        let mut y_max = f32::MIN;
        let mut y_min = f32::MAX;

        for symbol in (Range {
            start: 0 as char,
            end: 127 as char,
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
        if self.glyphs.is_empty() {
            panic!("Font is not initialized");
        }
        &self.glyphs[ch as usize]
    }
}
