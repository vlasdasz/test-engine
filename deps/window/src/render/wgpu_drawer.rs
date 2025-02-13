use crate::render::{
    background_pipeline::BackgroundPipeline, path_drawer::PathDrawer, sprite_drawer::PolygonPipeline,
};

#[derive(Default, Debug)]
pub struct WGPUDrawer {
    /// UI:
    pub path: PathDrawer,

    /// Sprites:
    pub background: BackgroundPipeline,

    pub polygon_test: PolygonPipeline,

    pub polygon: PolygonPipeline,
}
