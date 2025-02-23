use crate::{
    SpriteView,
    data::{RectInstance, RectView, UIRectInstance},
    pipelines::{pipeline_type::PipelineType, rect_pipeline::RectPipeline},
};

mod background_pipeline;
mod path_pipeline;
mod pipeline_type;
mod polygon_pipeline;
mod rect_pipeline;

const SPRITE_CODE: &str = include_str!("shaders/sprite.wgsl");
const TEXTURED_SPRITE_CODE: &str = include_str!("shaders/sprite_textured.wgsl");
const UI_CODE: &str = include_str!("shaders/ui_rect.wgsl");
const UI_IMAGE_CODE: &str = include_str!("shaders/ui_image.wgsl");
const UI_GRADIENT_CODE: &str = include_str!("shaders/ui_gradient.wgsl");

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

pub type UIGradientPipeline =
    RectPipeline<{ PipelineType::Color }, "ui_gradient", UI_GRADIENT_CODE, RectView, UIGradientInstance>;

pub use background_pipeline::BackgroundPipeline;
pub use path_pipeline::PathPipeline;
pub use polygon_pipeline::PolygonPipeline;

use crate::data::UIGradientInstance;
