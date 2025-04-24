use std::path::Path;

use anyhow::{Result, anyhow};
use gm::{Platform, flat::Size};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use log::trace;
use tiny_skia::Transform;
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, Origin3d, Sampler, SamplerDescriptor, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

use crate::Window;

#[derive(Debug)]
pub struct Texture {
    pub texture:  wgpu::Texture,
    pub view:     TextureView,
    pub sampler:  Sampler,
    pub size:     Size<u32>,
    pub channels: u8,
}

impl Texture {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn from_file_bytes(bytes: &[u8], label: &str) -> Result<Self> {
        if bytes.starts_with(b"<svg") {
            trace!("Loading SVG image: {label}");
            return Self::from_svg_image(bytes, label);
        }

        Ok(Self::from_dynamic_image(&image::load_from_memory(bytes)?, label))
    }

    pub fn from_raw_data(data: &[u8], size: Size<u32>, channels: u8, label: &str) -> Self {
        const RGBA_TEXTURE_FORMAT: TextureFormat = if Platform::ANDROID {
            TextureFormat::Rgba8Unorm
        } else {
            TextureFormat::Rgba8UnormSrgb
        };

        let extend_size = Extent3d {
            width:                 size.width,
            height:                size.height,
            depth_or_array_layers: 1,
        };

        let (channels, format) = match channels {
            1 => (1, TextureFormat::R8Unorm),
            3 | 4 => (4, RGBA_TEXTURE_FORMAT),
            ch => panic!("Invalid number of channels: {ch}"),
        };

        let device = Window::device();

        let texture = device.create_texture(&TextureDescriptor {
            label: label.into(),
            size: extend_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Window::queue().write_texture(
            TexelCopyTextureInfo {
                aspect:    TextureAspect::All,
                texture:   &texture,
                mip_level: 0,
                origin:    Origin3d::ZERO,
            },
            data,
            TexelCopyBufferLayout {
                offset:         0,
                bytes_per_row:  Some(u32::from(channels) * extend_size.width),
                rows_per_image: Some(extend_size.height),
            },
            extend_size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: "texture_sampler".into(),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size,
            channels,
        }
    }

    fn from_dynamic_image(img: &DynamicImage, label: &str) -> Self {
        let dimensions = img.dimensions();

        Self::from_raw_data(
            &img.to_rgba8(),
            (dimensions.0, dimensions.1).into(),
            img.color().channel_count(),
            label,
        )
    }

    fn from_svg_image(bytes: &[u8], label: &str) -> Result<Self> {
        use resvg::{
            render,
            tiny_skia::Pixmap,
            usvg::{Options, Tree},
        };

        let opt = Options::default();
        let tree = Tree::from_data(&bytes, &opt)?;

        let original_size = tree.size().to_int_size();

        let scale = 16.0;

        let width = (original_size.width() as f32 * scale).round() as u32;
        let height = (original_size.height() as f32 * scale).round() as u32;

        let mut pixmap = Pixmap::new(width, height).unwrap();

        let transform = Transform::from_scale(scale, scale);

        render(&tree, transform, &mut pixmap.as_mut());

        // _save_rgba_image(&pixmap.data(), width, height, "/home/vladas/svg.png")?;

        Ok(Self::from_raw_data(
            &pixmap.data(),
            dbg!((width, height).into()),
            4,
            label,
        ))
    }

    pub fn create_depth_texture(device: &Device, size: Size<u32>, label: &str) -> Self {
        let extend = Extent3d {
            width:                 size.width,
            height:                size.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label:           label.into(),
            size:            extend,
            mip_level_count: 1,
            sample_count:    1,
            dimension:       TextureDimension::D2,
            format:          Self::DEPTH_FORMAT,
            usage:           TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats:    &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: "depth_texture_sampler".into(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            compare: None, // doesn't work on iOS 12 Some(wgpu::CompareFunction::LessEqual), // 5.
            // compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size,
            channels: 1,
        }
    }
}

fn _save_rgba_image(buffer: &[u8], width: u32, height: u32, path: &str) -> Result<()> {
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, buffer.to_vec())
        .ok_or(anyhow!("Failed to create image buffer"))?;

    img.save(Path::new(path))?;
    Ok(())
}
