use gm::{
    flat::{Point, Shape, Size},
    Color,
};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use refs::Weak;
use vents::Event;
use wgpu_wrapper::image::Image;

use crate::Sprite;

#[derive(Default)]
pub struct SpriteData {
    pub(crate) position: Point,

    pub(crate) size:        Size,
    pub(crate) rotation:    f32,
    pub(crate) is_selected: bool,

    pub(crate) rigid_handle:    Option<RigidBodyHandle>,
    pub(crate) collider_handle: Option<ColliderHandle>,

    pub color: Color,

    pub image:        Weak<Image>,
    pub on_collision: Event<Weak<dyn Sprite>>,
}

impl SpriteData {
    pub fn make(shape: Shape, position: Point) -> Self {
        Self {
            position,
            size: shape.size(),
            ..Default::default()
        }
    }
}
