use std::sync::Arc;

use anyhow::Result;
use wgpu::{Adapter, Device, Instance, SurfaceConfiguration};
use winit::window::Window;

use crate::image::Texture;

pub(crate) struct Surface {
    pub presentable: wgpu::Surface<'static>,

    pub depth_texture: Texture,
}

impl Surface {
    pub fn new(
        instance: &Instance,
        adapter: &Adapter,
        device: &Device,
        config: &SurfaceConfiguration,
        window: Arc<Window>,
    ) -> Result<Self> {
        let surface = instance.create_surface(window.clone())?; // Android fail

        let _surface_caps = surface.get_capabilities(adapter);

        surface.configure(device, config);

        let depth_texture =
            Texture::create_depth_texture(device, (config.width, config.height).into(), "depth_texture");

        Ok(Self {
            presentable: surface,
            depth_texture,
        })
    }
}
