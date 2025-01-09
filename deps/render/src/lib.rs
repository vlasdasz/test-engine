#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use window::{SpriteInstance, SpriteView};

use crate::{rect_instance::RectInstance, rect_pipeline::RectPipeline, rect_view::RectView};

pub mod rect_instance;
mod rect_pipeline;
pub mod rect_view;

const SPRITE_CODE: &str = include_str!("sprite.wgsl");
const UI_CODE: &str = include_str!("rect.wgsl");

pub type SpriteBoxPipepeline = RectPipeline<"sprite_box", SPRITE_CODE, SpriteView, SpriteInstance>;

pub type UIRectPipepeline = RectPipeline<"ui_rect", UI_CODE, RectView, RectInstance>;
