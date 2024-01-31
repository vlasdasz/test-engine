use anyhow::Result;
use glyph_brush::{
    ab_glyph::FontArc, BrushAction, GlyphBrush, GlyphBrushBuilder, GlyphVertex, Section, Text,
};
use gm::flat::Point;
use wgpu::{
    AddressMode, Device, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, SamplerDescriptor,
    TextureAspect, TextureViewDescriptor,
};

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
        let channels = 1u32;

        let mut glyph_brush: GlyphBrush<Point> = GlyphBrushBuilder::using_font(self.font.clone()).build();

        glyph_brush.queue(Section::default().add_text(Text::new(&text)));

        let mut texture: Option<wgpu::Texture> = None;

        let mut width = 0;
        let mut height = 0;

        match glyph_brush.process_queued(
            |rectangle, texture_data| {
                width = rectangle.width();
                height = rectangle.height();

                let size = wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                };
                texture = device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: text.as_str().into(),
                        size,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::R8Uint,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    })
                    .into();

                queue.write_texture(
                    ImageCopyTexture {
                        aspect:    TextureAspect::All,
                        texture:   texture.as_ref().unwrap(),
                        mip_level: 0,
                        origin:    Origin3d::ZERO,
                    },
                    texture_data,
                    ImageDataLayout {
                        offset:         0,
                        bytes_per_row:  Some(channels * width),
                        rows_per_image: Some(height),
                    },
                    size,
                );
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

        let texture = texture.unwrap();

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Texture {
            texture,
            view,
            sampler,
            size: (width, height).into(),
            channels: 1,
        })
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
