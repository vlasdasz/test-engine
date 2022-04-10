use derivative::Derivative;
use gl_image::Image;
use gm::{
    flat::{Point, Shape},
    Color,
};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use rtools::{Event, IntoF32, Rglica};

use crate::{Level, Sprite};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct SpriteData {
    pub(crate) position:    Point,
    pub(crate) shape:       Shape,
    pub(crate) rotation:    f32,
    #[derivative(Debug = "ignore")]
    pub(crate) level:       Rglica<dyn Level>,
    pub(crate) is_selected: bool,

    pub(crate) rigid_handle:    Option<RigidBodyHandle>,
    pub(crate) collider_handle: Option<ColliderHandle>,

    pub tag:   String,
    pub color: Color,
    pub image: Option<Image>,

    pub on_collision: Event<Rglica<dyn Sprite>>,
}

impl SpriteData {
    pub(crate) fn with_level(mut self, level: Rglica<dyn Level>) -> Self {
        debug_assert!(level.is_ok());
        self.level = level;
        self
    }
}

impl<X: IntoF32, Y: IntoF32, W: IntoF32, H: IntoF32> From<(X, Y, W, H)> for SpriteData {
    fn from(data: (X, Y, W, H)) -> Self {
        Self {
            position: (data.0.into_f32(), data.1.into_f32()).into(),
            shape: (data.2.into_f32(), data.3.into_f32()).into(),
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

    fn make(shape: Shape, position: Point, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            shape,
            position,
            level,
            ..Default::default()
        })
    }
}
