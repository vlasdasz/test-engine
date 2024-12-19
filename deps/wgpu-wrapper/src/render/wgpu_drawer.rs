use gm::{Color, flat::Rect};
use wgpu::RenderPass;

use crate::{
    RectPipeline,
    render::{
        background_pipeline::BackgroundPipeline,
        image_drawer::ImageDrawer,
        old_rect_drawer::OldRectDrawer,
        path_drawer::PathDrawer,
        sprite_drawer::{BoxPipeline, PolygonPipeline, TexturedBoxPipeline},
    },
};

#[derive(Default, Debug)]
pub struct WGPUDrawer {
    /// UI:
    pub image:    ImageDrawer,
    pub path:     PathDrawer,
    pub rect:     RectPipeline,
    pub old_rect: OldRectDrawer,

    /// Sprites:
    pub background: BackgroundPipeline,

    pub polygon_test: PolygonPipeline,

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
            self.old_rect.draw(render_pass, &rect, color, z_position);
        }
    }
}
