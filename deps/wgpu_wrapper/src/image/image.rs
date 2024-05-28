use std::{fs::read, path::Path};

use anyhow::Result;
use gm::flat::Size;
use log::error;
use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};
use refs::Weak;
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
};

use crate::{image::Texture, WGPUApp};

#[derive(Debug)]
pub struct Image {
    pub size:        Size<u32>,
    pub channels:    u8,
    pub(crate) bind: BindGroup,
}

impl Image {
    fn load_to_wgpu(name: &str, data: &[u8]) -> Result<Self> {
        let texture = Texture::from_file_bytes(data, name)?;
        Self::from_texture(&texture)
    }

    pub fn from_texture(texture: &Texture) -> Result<Self> {
        let device = WGPUApp::device();

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("image_bind_group"),
            layout:  &Self::uniform_layout(),
            entries: &[
                BindGroupEntry {
                    binding:  0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding:  1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Ok(Self {
            size: texture.size,
            channels: texture.channels,
            bind,
        })
    }

    pub fn from_raw_data(
        data: &[u8],
        name: impl Into<String>,
        size: Size<u32>,
        channels: u8,
    ) -> Result<Weak<Image>> {
        let name = name.into();
        let texture = Texture::from_raw_data(data, size, channels, &name);
        let image = Self::from_texture(&texture)?;
        Ok(Image::add_with_name(&name, || image))
    }

    pub fn from_file_data(data: &[u8], name: &str) -> Weak<Image> {
        Image::add_with_name(name, || {
            Self::load_to_wgpu(name, data).expect("Failed to load image {name} to wgpu")
        })
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }
}

managed!(Image);

static DEFAULT_IMAGE_DATA: &[u8] = include_bytes!("delete.png");

impl ResourceLoader for Image {
    fn load_path(path: &Path) -> Self {
        let data = match read(path) {
            Ok(data) => data,
            Err(err) => {
                error!(
                    "Failed to read image file: {}. Error: {err} Returning default image",
                    path.display()
                );
                DEFAULT_IMAGE_DATA.into()
            }
        };
        Self::load_data(&data, path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let name = name.to_string();

        Image::load_to_wgpu(&name.to_string(), data)
            .unwrap_or_else(|err| panic!("Failed to load image {name} to wgpu. Err: {err}"))
    }
}

impl Image {
    pub(crate) fn uniform_layout() -> BindGroupLayout {
        WGPUApp::device().create_bind_group_layout(&BindGroupLayoutDescriptor {
            label:   "image_bind_group_layout".into(),
            entries: &[
                BindGroupLayoutEntry {
                    binding:    0,
                    visibility: ShaderStages::FRAGMENT,
                    ty:         BindingType::Texture {
                        multisampled:   false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type:    TextureSampleType::Float { filterable: true },
                    },
                    count:      None,
                },
                BindGroupLayoutEntry {
                    binding:    1,
                    visibility: ShaderStages::FRAGMENT,
                    ty:         BindingType::Sampler(SamplerBindingType::Filtering),
                    count:      None,
                },
            ],
        })
    }
}
