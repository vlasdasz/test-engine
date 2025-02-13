#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use window::SpriteView;

use crate::{rect_instance::RectInstance, rect_pipeline::RectPipeline, rect_view::RectView};

mod background_pipeline;
mod path_pipeline;
mod polygon_pipeline;
pub mod rect_instance;
mod rect_pipeline;
pub mod rect_view;
mod uniform;

const SPRITE_CODE: &str = include_str!("shaders/sprite.wgsl");
const TEXTURED_SPRITE_CODE: &str = include_str!("shaders/sprite_textured.wgsl");
const UI_CODE: &str = include_str!("shaders/rect.wgsl");
const UI_IMAGE_CODE: &str = include_str!("shaders/ui_image.wgsl");

pub type SpriteBoxPipepeline = RectPipeline<false, "sprite_box", SPRITE_CODE, SpriteView, RectInstance>;
pub type TexturedSpriteBoxPipeline =
    RectPipeline<true, "textured_sprite_box", TEXTURED_SPRITE_CODE, SpriteView, RectInstance>;

pub type UIRectPipepeline = RectPipeline<false, "ui_rect", UI_CODE, RectView, RectInstance>;
pub type UIImageRectPipepeline = RectPipeline<true, "ui_image_rect", UI_IMAGE_CODE, RectView, RectInstance>;

pub use background_pipeline::BackgroundPipeline;
pub use path_pipeline::PathPipeline;
pub use polygon_pipeline::PolygonPipeline;
