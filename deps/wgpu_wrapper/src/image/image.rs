use std::path::Path;

use anyhow::Result;
use gm::flat::IntSize;
use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};
use refs::Weak;
use rtools::file::File;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
    SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
};

use crate::{image::Texture, render::state::State, WGPUApp};

#[derive(Debug)]
pub struct Image {
    pub size:        IntSize,
    pub channels:    u8,
    pub(crate) bind: BindGroup,
}

impl Image {
    fn load_to_wgpu(state: &State, name: &str, data: &[u8]) -> Result<Self> {
        let texture = Texture::from_file_bytes(&state.drawer.device, &state.drawer.queue, data, name)?;
        Self::from_texture(texture, &state.drawer.device)
    }

    pub fn from_texture(texture: Texture, device: &Device) -> Result<Self> {
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("diffuse_bind_group"),
            layout:  &Self::bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding:  1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
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
        state: &State,
        data: &[u8],
        name: String,
        size: IntSize,
        channels: u8,
    ) -> Result<Weak<Image>> {
        let texture = Texture::from_raw_data(
            &state.drawer.device,
            &state.drawer.queue,
            data,
            size,
            channels,
            &name,
        );
        let image = Self::from_texture(texture, &state.drawer.device)?;
        Ok(Image::add_with_name(&name, || image))
    }

    pub fn from_file_data(data: &[u8], name: String) -> Weak<Image> {
        Image::add_with_name(&name.clone(), || {
            Self::load_to_wgpu(&WGPUApp::current().state, &name, data)
                .expect("Failed to load image {name} to wgpu")
        })
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }
}

managed!(Image);

impl ResourceLoader for Image {
    fn load_path(path: &Path) -> Self {
        Self::load_data(&File::read(path), path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let name = name.to_string();

        Image::load_to_wgpu(&WGPUApp::current().state, &name.to_string(), data)
            .unwrap_or_else(|err| panic!("Failed to load image {name} to wgpu. Err: {err}"))
    }
}

impl Image {
    pub(crate) fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
