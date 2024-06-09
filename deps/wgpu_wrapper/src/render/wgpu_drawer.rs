use std::ops::Range;

use anyhow::Result;
use gm::{flat::Rect, Color};
use wgpu::{BindGroup, Buffer, RenderPass};

use crate::{
    image::Image,
    render::{
        image_drawer::ImageDrawer,
        path_drawer::PathDrawer,
        rect_drawer::RectDrawer,
        sprite_drawer::{SpriteDrawer, TexturedSpriteDrawer},
    },
    state::TEXTURE_FORMAT,
};

#[derive(Debug)]
pub struct WGPUDrawer {
    image_drawer:           ImageDrawer,
    pub(crate) path_drawer: PathDrawer,

    pub rect_drawer:            RectDrawer,
    pub sprite_drawer:          SpriteDrawer,
    pub textured_sprite_drawer: TexturedSpriteDrawer,
}

impl WGPUDrawer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            rect_drawer:  RectDrawer::new(TEXTURE_FORMAT),
            image_drawer: ImageDrawer::new(TEXTURE_FORMAT),
            path_drawer:  PathDrawer::new(TEXTURE_FORMAT),

            sprite_drawer:          SpriteDrawer::new(TEXTURE_FORMAT),
            textured_sprite_drawer: TexturedSpriteDrawer::new(TEXTURE_FORMAT),
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
            self.rect_drawer.draw(render_pass, &rect, color, z_position);
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
            .draw_buffer(render_pass, rect, buffer, bind_group, vertex_range, z_position);
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
