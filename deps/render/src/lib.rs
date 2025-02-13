#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use window::SpriteView;

use crate::{rect_instance::RectInstance, rect_pipeline::RectPipeline, rect_view::RectView};

pub mod rect_instance;
mod rect_pipeline;
pub mod rect_view;

const SPRITE_CODE: &str = include_str!("sprite.wgsl");
const TEXTURED_SPRITE_CODE: &str = include_str!("sprite_textured.wgsl");
const UI_CODE: &str = include_str!("rect.wgsl");

pub type SpriteBoxPipepeline = RectPipeline<false, "sprite_box", SPRITE_CODE, SpriteView, RectInstance>;
pub type TexturedSpriteBoxPipeline =
    RectPipeline<true, "textured_sprite_box", TEXTURED_SPRITE_CODE, SpriteView, RectInstance>;

pub type UIRectPipepeline = RectPipeline<false, "ui_rect", UI_CODE, RectView, RectInstance>;
