#![allow(incomplete_features)]
#![feature(box_into_inner)]
#![feature(arbitrary_self_types)]
#![feature(specialization)]

extern crate core;

mod control;
mod event_handler;
mod level;
mod level_manager;
mod sets;
mod sprite_data;
mod to_collider;
mod units;

pub use control::Control;
pub use level::{Level, LevelBase, LevelCreation, LevelInternal, LevelSetup, LevelTemplates};
pub use level_manager::LevelManager;
pub use level_proc::level;
pub use rapier2d::dynamics::CoefficientCombineRule;
pub use sprite_data::SpriteData;
pub use to_collider::ToCollider;
pub use units::*;
