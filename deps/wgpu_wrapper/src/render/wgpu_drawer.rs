use std::ops::Range;

use anyhow::Result;
use gm::{
    flat::{Rect, Size},
    Color,
};
use wgpu::{BindGroup, Buffer, RenderPass, TextureFormat};

use crate::{
    image::Image,
    render::{
        image_drawer::ImageDrawer, instanced_sprite_drawer::InstancedSpriteDrawer, path_drawer::PathDrawer,
        rect_drawer::RectDrawer, sprite_drawer::SpriteDrawer,
    },
};

#[derive(Debug)]
pub struct WGPUDrawer {
    pub window_size:             Size,
    rect_drawer:                 RectDrawer,
    image_drawer:                ImageDrawer,
    pub sprite_drawer:           SpriteDrawer,
    pub instanced_sprite_drawer: InstancedSpriteDrawer,
    pub(crate) path_drawer:      PathDrawer,
}

impl WGPUDrawer {
    pub fn new(texture_format: TextureFormat) -> Result<Self> {
        Ok(Self {
            window_size:             Default::default(),
            rect_drawer:             RectDrawer::new(texture_format),
            path_drawer:             PathDrawer::new(texture_format),
            sprite_drawer:           SpriteDrawer::new(texture_format),
            instanced_sprite_drawer: InstancedSpriteDrawer::new(texture_format),
            image_drawer:            ImageDrawer::new(),
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
        self.rect_drawer.draw(render_pass, rect, color, z_position);
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
            self.rect_drawer.draw(render_pass, &rect, color, z_position)
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
        self.path_drawer
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
        self.image_drawer.draw(image, rect, render_pass, vertices, z_position);
    }
}
