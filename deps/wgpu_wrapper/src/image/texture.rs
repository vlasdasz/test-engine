use anyhow::Result;
use gm::flat::Size;
use image::{DynamicImage, GenericImageView};
use wgpu::{
    AddressMode, Device, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Sampler,
    SamplerDescriptor, TextureAspect, TextureView, TextureViewDescriptor,
};

#[derive(Debug)]
pub struct Texture {
    pub texture:  wgpu::Texture,
    pub view:     TextureView,
    pub sampler:  Sampler,
    pub size:     Size,
    pub channels: u32,
}

impl Texture {
    pub fn from_bytes(device: &Device, queue: &Queue, bytes: &[u8], label: &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, label)
    }

    fn from_image(device: &Device, queue: &Queue, img: &DynamicImage, label: &str) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let channels = img.color().channel_count();

        let size = wgpu::Extent3d {
            width:                 dimensions.0,
            height:                dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.into(),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                aspect:    TextureAspect::All,
                texture:   &texture,
                mip_level: 0,
                origin:    Origin3d::ZERO,
            },
            &rgba,
            ImageDataLayout {
                offset:         0,
                bytes_per_row:  Some(channels as u32 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        dbg!(&view);

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: "texture_sampler".into(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        dbg!(&sampler);

        Ok(Self {
            texture,
            view,
            sampler,
            size: (dimensions.0, dimensions.1).into(),
            channels: channels.into(),
        })
    }
}
