use std::path::PathBuf;
use crate::gm::{Size, Point};
use crate::image::Image;
use std::ffi::c_void;
use crate::ui::Glyph;
use std::ops::Range;
use std::fs;


fn render_glyph(font: &fontdue::Font, symbol: char) -> Glyph {

    let (metrics, bitmap) = font.rasterize(symbol, 48.0);

    let size = Size {
        width: metrics.width as f32,
        height: metrics.height as f32
    };

    let image = Image::from(bitmap.as_ptr() as *const c_void, size, 1);

    Glyph::new(symbol, image, 20, Point::new())
}

#[derive(Debug, Clone)]
pub struct Font {

    pub size: u32,
    pub height: f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>
}

impl Font {

    pub fn blank() -> Font {
        Font { size: 0, height: 0.0, baseline_shift: 0.0, glyphs: vec![] }
    }

    pub fn new(path: &PathBuf, size: u32) -> Option<Font> {

        let data = fs::read(path).unwrap();
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap();

        let mut glyphs = Vec::<Glyph>::with_capacity(128);

        let mut y_max: f32 = f32::MIN;
        let mut y_min: f32 = f32::MAX;

        for symbol in (Range { start: 0 as char, end: 127 as char }) {
            let glyph = render_glyph(&font, symbol);
            if y_max < glyph.y_max() { y_max = glyph.y_max() }
            if y_min > glyph.y_min() { y_min = glyph.y_min() }
            glyphs.push(glyph);
        }

        let height = y_max - y_min;
        let baseline_position = y_min.abs();
        let baseline_shift = height / 2.0 - baseline_position;

        Some(Font { size, height, baseline_shift, glyphs })
    }
}

impl Font {

    pub fn glyph_for_char(&self, ch: char) -> &Glyph {
        &self.glyphs[ch as usize]
    }
}
