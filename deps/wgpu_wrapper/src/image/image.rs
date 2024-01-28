use std::path::Path;

use anyhow::Result;
use gm::flat::Size;
use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};
use refs::Weak;
use rtools::file::File;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
    SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
};

use crate::{app::App, image::Texture};

// static GET_STATE: Mutex<RefCell<Option<Box<dyn Fn() -> (&'static Queue,
// &'static Device)>>>> =     Mutex::new(RefCell::new(None));

#[derive(Debug)]
pub struct Image {
    pub size:        Size,
    pub channels:    u32,
    pub(crate) bind: BindGroup,
}

impl Image {
    fn load_to_wgpu(name: &str, data: &[u8]) -> Result<Image> {
        let state = &App::current().state;

        let texture = Texture::from_bytes(&state.device, &state.queue, data, name)?;

        let bind = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &Self::bind_group_layout(&state.device),
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
            label:   Some("diffuse_bind_group"),
        });

        Ok(Image {
            size: texture.size,
            channels: texture.channels,
            bind,
        })
    }

    pub fn from(data: &[u8], name: String) -> Weak<Image> {
        Image::add_with_name(&name.clone(), || {
            Self::load_to_wgpu(&name, data).expect("Failed to load image {name} to wgpu")
        })
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }
}

impl Image {
    // pub fn render(name: impl ToString, size: impl Into<Size>, draw: impl
    // FnOnce(&mut Image)) -> Weak<Image> {     let name = name.to_string();
    //
    //     if let Some(image) = Image::weak_with_name(&name) {
    //         return image;
    //     }
    //
    //     let size = size.into();
    //     let buffer = FrameBuffer::from(size);
    //
    //     buffer.bind();
    //
    //     let mut image = Self {
    //         size,
    //         channels: 4,
    //         flipped: false,
    //         flipped_y: false,
    //         buffer,
    //         total_size: size_of::<Self>() + 10,
    //         name: name.clone(),
    //     };
    //
    //     GLWrapper::clear_with_color(Color::CLEAR);
    //
    //     draw(&mut image);
    //
    //     GLWrapper::unbind_framebuffer();
    //
    //     Image::add_with_name(name, image)
    // }
    //
    // pub fn render_path(name: impl ToString, color: Color, path: Points,
    // draw_mode: DrawMode) -> Weak<Image> {     let size = path.max_size();
    //
    //     let path = initialize_path_data(path, &color, draw_mode);
    //
    //     dbg!(&size);
    //
    //     Self::render(name, size, |image| {
    //         GLWrapper::set_viewport(size);
    //         GLWrapper::clear_with_color(Color::RED.with_alpha(0.0));
    //         BasicShaders::path().enable().set_color(&color).set_size(size);
    //         path.buffer.draw_with_mode(path.draw_mode.to_gl());
    //
    //         dbg!(GLWrapper::read_pixel((5, 5).into()));
    //         dbg!(GLWrapper::read_pixel((1, 1).into()));
    //         dbg!(GLWrapper::read_pixel((0, 0).into()));
    //         image.flipped_y = true;
    //     })
    // }
}

managed!(Image);

impl ResourceLoader for Image {
    fn load_path(path: &Path) -> Self {
        Self::load_data(&File::read(path), path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let name = name.to_string();

        Image::load_to_wgpu(&name.to_string(), data)
            .unwrap_or_else(|err| panic!("Failed to load image {name} to wgpu. Err: {err}"))
    }
}

// pub fn draw_image(image: &Image, rect: &Rect, color: &Color, priority: usize,
// is_text: bool) {     if image.is_invalid() {
//         return;
//     }
//
//     if is_text {
//         ImageShaders::text().enable()
//     } else if image.is_monochrome() {
//         ImageShaders::mono().enable().set_color(color)
//     } else {
//         ImageShaders::color().enable()
//     }
//     .set_flipped(image.flipped)
//     .set_flipped_y(image.flipped_y)
//     .set_priority(priority);
//
//     GLWrapper::set_viewport(*rect);
//
//     image.bind();
//     Buffers::get().full_image.draw();
// }

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
