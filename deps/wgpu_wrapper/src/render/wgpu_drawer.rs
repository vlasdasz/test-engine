use std::ops::Range;

use anyhow::Result;
use gm::{
    flat::{Rect, Size},
    Color,
};
use wgpu::{BindGroup, Buffer, RenderPass, TextureFormat};

use crate::{
    image::Image,
    render::{image_state::ImageState, path_state::PathState, rect_state::RectState},
};

#[derive(Debug)]
pub struct WGPUDrawer {
    pub window_size:       Size,
    rect_state:            RectState,
    colored_image_state:   ImageState,
    pub(crate) path_state: PathState,
}

impl WGPUDrawer {
    pub fn new(texture_format: TextureFormat) -> Result<Self> {
        let rect_state = RectState::new(texture_format);
        let path_state = PathState::new(texture_format);
        let colored_image_state = ImageState::new();
        Ok(Self {
            window_size: Default::default(),
            rect_state,
            path_state,
            colored_image_state,
        })
    }
}

impl WGPUDrawer {
    pub fn fill_rect<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        z_position: f32,
    ) {
        self.rect_state.draw(render_pass, rect, color, z_position);
    }

    pub fn outline_rect<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        z_position: f32,
        width: f32,
    ) {
        for rect in rect.to_borders(width) {
            self.rect_state.draw(render_pass, &rect, color, z_position)
        }
    }

    pub fn draw_buffer<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        buffer: &'a Buffer,
        bind_group: &'a BindGroup,
        vertex_range: Range<u32>,
        z_position: f32,
    ) {
        self.path_state
            .draw_buffer(render_pass, rect, buffer, bind_group, vertex_range, z_position)
    }

    pub fn draw_image<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        image: &'static Image,
        rect: &Rect,
        vertices: Option<&'a Buffer>,
        z_position: f32,
    ) {
        self.colored_image_state.draw(image, rect, render_pass, vertices, z_position);
    }
}
