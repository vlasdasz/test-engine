#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

pub use crate::shader_data::SpriteView;
use crate::{pipelines::rect_pipeline::RectPipeline, rect_instance::RectInstance, rect_view::RectView};

mod device_helper;
mod path_data;
mod pipelines;
pub mod rect_instance;
pub mod rect_view;
mod shader_data;
mod uniform;
mod vec_buffer;
mod vertex_layout;

const SPRITE_CODE: &str = include_str!("pipelines/shaders/sprite.wgsl");
const TEXTURED_SPRITE_CODE: &str = include_str!("pipelines/shaders/sprite_textured.wgsl");
const UI_CODE: &str = include_str!("pipelines/shaders/rect.wgsl");
const UI_IMAGE_CODE: &str = include_str!("pipelines/shaders/ui_image.wgsl");

pub type SpriteBoxPipepeline = RectPipeline<false, "sprite_box", SPRITE_CODE, SpriteView, RectInstance>;
pub type TexturedSpriteBoxPipeline =
    RectPipeline<true, "textured_sprite_box", TEXTURED_SPRITE_CODE, SpriteView, RectInstance>;

pub type UIRectPipepeline = RectPipeline<false, "ui_rect", UI_CODE, RectView, RectInstance>;
pub type UIImageRectPipepeline = RectPipeline<true, "ui_image_rect", UI_IMAGE_CODE, RectView, RectInstance>;

pub use path_data::PathData;
pub use pipelines::{
    background_pipeline::BackgroundPipeline, path_pipeline::PathPipeline, polygon_pipeline::PolygonPipeline,
};
