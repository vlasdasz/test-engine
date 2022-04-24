#![feature(box_into_inner)]
#![feature(explicit_generic_args_with_impl_trait)]

extern crate core;

mod body;
mod control;
mod level;
mod level_base;
mod level_creation;
mod sets;
mod sprite;
mod sprite_data;
mod sprites_drawer;
mod to_collider;
mod units;
mod wall;

pub use body::Body;
pub use control::Control;
pub use level::Level;
pub use level_base::{LevelBase, LevelTemplates};
pub use level_creation::LevelCreation;
pub use sprite::{Sprite, SpriteTemplates};
pub use sprite_data::SpriteData;
pub use sprites_drawer::SpritesDrawer;
pub use to_collider::ToCollider;
pub use units::{Player, Unit, Weapon};
pub use wall::Wall;
