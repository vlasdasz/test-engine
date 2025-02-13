use crate::render::{
    background_pipeline::BackgroundPipeline, image_drawer::ImageDrawer, path_drawer::PathDrawer,
    sprite_drawer::PolygonPipeline,
};

#[derive(Default, Debug)]
pub struct WGPUDrawer {
    /// UI:
    pub image: ImageDrawer,
    pub path:  PathDrawer,

    /// Sprites:
    pub background: BackgroundPipeline,

    pub polygon_test: PolygonPipeline,

    pub polygon: PolygonPipeline,
}
