use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Mutex,
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

static RENDER: Mutex<bool> = Mutex::new(true);

pub static DEFAULT_FONT_SIZE: f32 = 64.0;

fn render_glyph(font: &fontdue::Font, symbol: char, size: f32) -> Glyph {
    let (metrics, bitmap) = font.rasterize(symbol, size);

    let size = Size {
        width:  metrics.width as f32,
        height: metrics.height as f32,
    };

    let image = Image::from(bitmap.as_ptr().cast(), size, 1, hash(symbol));

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
    pub size:           f32,
    pub height:         f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>,
}

impl Font {
    fn from_data(path: &Path, data: &[u8], size: f32) -> Result<Font, &'static str> {
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
            path: path.into(),
            font,
            size,
            height,
            baseline_shift,
            glyphs,
        })
    }

    fn new(path: &Path, size: f32) -> Result<Font, &'static str> {
        trace!("Loading font {:?}", path);
        Self::from_data(path, &File::read(path), size)
    }
}

impl Font {
    pub fn san_francisco() -> Handle<Self> {
        const SF: &str = "default_sf";

        if let Some(sf) = Font::handle_with_name(SF) {
            return sf;
        }

        let sf = Font::from_data(
            &PathBuf::from(SF),
            include_bytes!("fonts/SF.otf"),
            DEFAULT_FONT_SIZE,
        )
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

impl LoadFromPath for Font {
    fn load(path: &Path) -> Self {
        Font::new(path, DEFAULT_FONT_SIZE).unwrap()
    }
}

managed!(Font);
