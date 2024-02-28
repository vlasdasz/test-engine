use std::ops::Range;

use anyhow::Result;
use gm::{
    flat::{Rect, Size},
    Color,
};
use wgpu::{BindGroup, Buffer, Device, PolygonMode, Queue, RenderPass, TextureFormat};

use crate::{
    image::Image,
    render::{image_state::ImageState, path_state::PathState, rect_state::RectState},
};

#[derive(Debug)]
pub struct WGPUDrawer {
    pub window_size:       Size,
    pub device:            Device,
    pub queue:             Queue,
    rect_state:            RectState,
    colored_image_state:   ImageState,
    pub(crate) path_state: PathState,
}

impl WGPUDrawer {
    pub fn new(device: Device, queue: Queue, texture_format: TextureFormat) -> Result<Self> {
        let rect_state = RectState::new(&device, texture_format);
        let path_state = PathState::new(&device, texture_format);
        let colored_image_state = ImageState::new(&device);
        Ok(Self {
            window_size: Default::default(),
            device,
            queue,
            rect_state,
            path_state,
            colored_image_state,
        })
    }
}

impl WGPUDrawer {
    pub fn draw_rect<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        polygon_mode: PolygonMode,
        z_position: f32,
    ) {
        self.rect_state
            .draw(&self.device, render_pass, rect, color, polygon_mode, z_position);
    }

    pub fn draw_buffer<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        polygon_mode: PolygonMode,
        buffer: &'a Buffer,
        bind_group: &'a BindGroup,
        vertex_range: Range<u32>,
        z_position: f32,
    ) {
        self.path_state.draw_buffer(
            device,
            render_pass,
            rect,
            polygon_mode,
            buffer,
            bind_group,
            vertex_range,
            z_position,
        )
    }

    pub fn draw_image<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        image: &'static Image,
        rect: &Rect,
        vertices: Option<&'a Buffer>,
        z_position: f32,
    ) {
        self.colored_image_state
            .draw(device, image, rect, render_pass, vertices, z_position);
    }
}
