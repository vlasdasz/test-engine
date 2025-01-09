#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use window::{SpriteInstance, SpriteView};

use crate::rect_pipeline::RectPipeline;

mod rect_pipeline;

static SPRITE_SHADER: &str = include_str!("sprite.wgsl");

pub type BoxPipeline = RectPipeline<"sprite_box", SPRITE_SHADER, SpriteView, SpriteInstance>;
