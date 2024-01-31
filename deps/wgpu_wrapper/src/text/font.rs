use anyhow::Result;
use glyph_brush::{
    ab_glyph::FontArc, BrushAction, GlyphBrush, GlyphBrushBuilder, GlyphVertex, Section, Text,
};
use gm::flat::Point;
use wgpu::{Device, Queue};

use crate::image::Texture;

#[derive(Debug)]
pub struct Font {
    font: FontArc,
}

impl Font {
    pub fn from_data(_name: impl ToString, data: &'static [u8]) -> Result<Self> {
        let font = FontArc::try_from_slice(data)?;
        Ok(Font { font })
    }

    pub fn draw(&self, device: &Device, queue: &Queue, text: impl ToString) -> Result<Texture> {
        let text = text.to_string();

        let mut glyph_brush: GlyphBrush<Point> = GlyphBrushBuilder::using_font(self.font.clone()).build();

        glyph_brush.queue(Section::default().add_text(Text::new(&text).with_scale(80.0)));

        let mut texture: Option<Texture> = None;

        let mut width = 0;
        let mut height = 0;

        match glyph_brush.process_queued(
            |rectangle, texture_data| {
                width = rectangle.width();
                height = rectangle.height();

                texture =
                    Texture::from_raw_data(device, queue, texture_data, (width, height).into(), 1, &text)
                        .into();
            },
            |glyph_vertex: GlyphVertex| {
                dbg!(&glyph_vertex);
                Point::default()
            },
        )? {
            BrushAction::Draw(draw) => {
                dbg!(&draw);
            }
            BrushAction::ReDraw => {}
        };

        Ok(texture.unwrap())
    }
}

const SF: &str = "default_helvetica";

impl Font {
    pub fn helvetice() -> Result<Self> {
        Self::from_data(SF, include_bytes!("fonts/Helvetica.ttf"))
    }
}

#[test]
fn test_font() -> Result<()> {
    const SF: &str = "default_helvetica";

    let _font = Font::from_data(SF, include_bytes!("fonts/Helvetica.ttf"))?;

    Ok(())
}
