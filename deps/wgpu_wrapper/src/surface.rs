use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::info;
use wgpu::{CompositeAlphaMode, Device, PresentMode, Queue};
use winit::window::Window;

use crate::{image::Texture, state::TEXTURE_FORMAT};

pub(crate) struct Surface {
    pub config:      wgpu::SurfaceConfiguration,
    pub presentable: wgpu::Surface<'static>,

    pub device: Device,
    pub queue:  Queue,

    pub depth_texture: Texture,
}

impl Surface {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?; // Android fail

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or(anyhow!("Failed to request adapter"))?;

        let info = adapter.get_info();

        info!("{}", &info.backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(), //wgpu::Features::POLYGON_MODE_LINE,
                    required_limits:   if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label:             None,
                },
                None,
            )
            .await?;

        let _surface_caps = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format:       TEXTURE_FORMAT,
            width:        size.width,
            height:       size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode:   CompositeAlphaMode::Auto,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture =
            Texture::create_depth_texture(&device, (config.width, config.height).into(), "depth_texture");

        Ok(Self {
            presentable: surface,
            config,
            device,
            queue,
            depth_texture,
        })
    }
}
