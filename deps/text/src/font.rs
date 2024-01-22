use std::{
    collections::HashMap,
    ops::Range,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

use gl_image::Image;
use gm::flat::Size;
use log::trace;
use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};
use refs::Weak;
use rtools::{file::File, hash};

use crate::glyph::Glyph;

static RENDER: AtomicBool = AtomicBool::new(true);

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
        metrics.advance_width,
        (metrics.bounds.xmin, metrics.bounds.height).into(),
    )
}

#[derive(Clone, Debug)]
pub struct Font {
    pub name:           String,
    pub font:           fontdue::Font,
    pub height:         f32,
    pub baseline_shift: f32,

    size:   f32,
    glyphs: HashMap<char, Glyph>,
}

impl Font {
    fn from_data(name: impl ToString, data: &[u8], size: f32) -> Result<Font, &'static str> {
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())?;

        let mut glyphs_map = HashMap::new();

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

                glyphs_map.insert(symbol, glyph);
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
            glyphs: glyphs_map,
        })
    }
}

impl Font {
    pub fn helvetica() -> Weak<Self> {
        const SF: &str = "default_helvetica";

        if let Some(sf) = Font::weak_with_name(SF) {
            return sf;
        }

        let sf = Font::from_data(SF, include_bytes!("fonts/Helvetica.ttf"), DEFAULT_FONT_SIZE)
            .expect("BUG: Failed to render default font");

        Font::add_with_name(SF, sf)
    }

    pub fn disable_render() {
        RENDER.store(false, Ordering::Relaxed);
    }

    pub fn render_enabled() -> bool {
        RENDER.load(Ordering::Relaxed)
    }

    pub fn glyph_for_char(&mut self, ch: char) -> &Glyph {
        debug_assert!(!self.glyphs.is_empty(), "Font is not initialized");
        self.glyphs.entry(ch).or_insert_with(|| render_glyph(&self.font, ch, self.size))
    }
}

impl ResourceLoader for Font {
    fn load_path(path: &Path) -> Self {
        Self::load_data(&File::read(path), path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        trace!("Loading font {:?}", name.to_string());
        Self::from_data(name, data, DEFAULT_FONT_SIZE).unwrap()
    }
}

managed!(Font);
