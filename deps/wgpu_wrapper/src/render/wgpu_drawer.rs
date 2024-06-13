use anyhow::Result;
use gm::{flat::Rect, Color};
use wgpu::RenderPass;

use crate::{
    render::{
        background_pipeline::BackgroundPipeline,
        image_drawer::ImageDrawer,
        path_drawer::PathDrawer,
        rect_drawer::RectDrawer,
        sprite_drawer::{BoxPipeline, PolygonPipeline, TexturedBoxPipeline},
    },
    state::TEXTURE_FORMAT,
};

#[derive(Default, Debug)]
pub struct WGPUDrawer {
    /// UI:
    pub image: ImageDrawer,
    pub path:  PathDrawer,
    pub rect:  RectDrawer,

    /// Sprites:
    pub background: BackgroundPipeline,

    pub polygon:      PolygonPipeline,
    pub sprite_box:   BoxPipeline,
    pub textured_box: TexturedBoxPipeline,
}

impl WGPUDrawer {
    pub fn outline_rect<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        z_position: f32,
        width: f32,
    ) {
        for rect in rect.to_borders(width) {
            self.rect.draw(render_pass, &rect, color, z_position);
        }
    }
}
