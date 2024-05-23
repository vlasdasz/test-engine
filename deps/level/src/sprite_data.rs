use gm::{
    flat::{Point, Rect, Shape},
    Color, ToF32,
};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use refs::{Own, Weak};
use vents::Event;
use wgpu_wrapper::image::Image;

use crate::{Level, Sprite};

#[derive(Default)]
pub struct SpriteData {
    pub(crate) position: Point,

    pub(crate) shape:       Shape,
    pub(crate) rotation:    f32,
    pub(crate) level:       Weak<dyn Level>,
    pub(crate) is_selected: bool,

    pub(crate) rigid_handle:    Option<RigidBodyHandle>,
    pub(crate) collider_handle: Option<ColliderHandle>,

    pub tag:   String,
    pub color: Color,

    pub image:        Weak<Image>,
    pub on_collision: Event<Weak<dyn Sprite>>,
}

impl<X: ToF32, Y: ToF32, W: ToF32, H: ToF32> From<(X, Y, W, H)> for SpriteData {
    fn from(data: (X, Y, W, H)) -> Self {
        let rect: Rect = (data).into();
        Self {
            position: rect.origin,
            shape: Shape::Rect(rect.size),
            color: Color::random(),
            ..Default::default()
        }
    }
}

impl Sprite for SpriteData {
    fn data(&self) -> &SpriteData {
        self
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        self
    }

    fn make(shape: Shape, position: Point, level: Weak<dyn Level>) -> Own<Self>
    where Self: Sized {
        Own::new(Self {
            position,
            shape,
            level,
            ..Default::default()
        })
    }
}
