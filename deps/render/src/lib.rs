#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use window::{SpriteInstance, SpriteView};

use crate::pipeline::Pipeline;

mod pipeline;

pub type BoxPipeline = Pipeline<"sprite", SpriteView, SpriteInstance>;
