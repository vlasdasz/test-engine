use anyhow::Result;
use gm::flat::IntSize;
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
    pub size:     IntSize,
    pub channels: u8,
}

impl Texture {
    pub fn from_file_bytes(device: &Device, queue: &Queue, bytes: &[u8], label: &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Ok(Self::from_dynamic_image(device, queue, &img, label))
    }

    pub fn from_raw_data(
        device: &Device,
        queue: &Queue,
        data: &[u8],
        size: IntSize,
        channels: u8,
        label: &str,
    ) -> Self {
        let extend_size = wgpu::Extent3d {
            width:                 size.width,
            height:                size.height,
            depth_or_array_layers: 1,
        };

        let format = match channels {
            1 => wgpu::TextureFormat::R8Unorm,
            4 => wgpu::TextureFormat::Rgba8UnormSrgb,
            ch => panic!("Invalid number of channels: {ch}"),
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.into(),
            size: extend_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
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
            data,
            ImageDataLayout {
                offset:         0,
                bytes_per_row:  Some(u32::from(channels) * extend_size.width),
                rows_per_image: Some(extend_size.height),
            },
            extend_size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

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

        Self {
            texture,
            view,
            sampler,
            size,
            channels,
        }
    }

    fn from_dynamic_image(device: &Device, queue: &Queue, img: &DynamicImage, label: &str) -> Self {
        let dimensions = img.dimensions();

        Self::from_raw_data(
            device,
            queue,
            &img.to_rgba8(),
            (dimensions.0, dimensions.1).into(),
            img.color().channel_count(),
            label,
        )
    }
}
