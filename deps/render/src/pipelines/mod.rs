use crate::{
    SpriteView,
    pipelines::{pipeline_type::PipelineType, rect_pipeline::RectPipeline},
    rect_instance::RectInstance,
    rect_view::RectView,
    ui_rect_instance::UIRectInstance,
};

mod background_pipeline;
mod path_pipeline;
mod pipeline_type;
mod polygon_pipeline;
mod rect_pipeline;

const SPRITE_CODE: &str = include_str!("shaders/sprite.wgsl");
const TEXTURED_SPRITE_CODE: &str = include_str!("shaders/sprite_textured.wgsl");
const UI_CODE: &str = include_str!("shaders/rect.wgsl");
const UI_IMAGE_CODE: &str = include_str!("shaders/ui_image.wgsl");

pub type SpriteBoxPipepeline =
    RectPipeline<{ PipelineType::Color }, "sprite_box", SPRITE_CODE, SpriteView, RectInstance>;
pub type TexturedSpriteBoxPipeline = RectPipeline<
    { PipelineType::Image },
    "textured_sprite_box",
    TEXTURED_SPRITE_CODE,
    SpriteView,
    RectInstance,
>;

pub type UIRectPipepeline =
    RectPipeline<{ PipelineType::Color }, "ui_rect", UI_CODE, RectView, UIRectInstance>;

pub type UIImageRectPipepeline =
    RectPipeline<{ PipelineType::Image }, "ui_image_rect", UI_IMAGE_CODE, RectView, UIRectInstance>;

pub use background_pipeline::BackgroundPipeline;
pub use path_pipeline::PathPipeline;
pub use polygon_pipeline::PolygonPipeline;
