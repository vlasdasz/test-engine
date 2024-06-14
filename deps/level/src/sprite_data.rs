use educe::Educe;
use gm::{
    flat::{Point, Shape, Size},
    Color,
};
use refs::Weak;
use vents::Event;
use wgpu_wrapper::image::Image;

use crate::Sprite;

#[derive(Educe)]
#[educe(Default)]
pub struct SpriteData {
    pub(crate) position: Point,

    pub(crate) size:        Size,
    pub(crate) rotation:    f32,
    pub(crate) is_selected: bool,

    pub(crate) collision_enabled: bool,

    pub shape: Shape,

    pub tag: u32,

    #[educe(Default = Color::random())]
    pub color: Color,

    pub image:        Weak<Image>,
    pub on_collision: Event<Weak<dyn Sprite>>,
}

impl SpriteData {
    pub fn make(shape: Shape, position: Point) -> Self {
        Self {
            position,
            size: shape.size(),
            shape,
            ..Default::default()
        }
    }
}
