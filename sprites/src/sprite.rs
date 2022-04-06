use std::fmt::Debug;

use gl_image::Image;
use gm::{
    flat::{Point, Shape, Size},
    Color,
};
use rapier2d::{geometry::Collider, prelude::RigidBody};
use rtools::{address::Address, Rglica};

use crate::{Level, SpriteData};

pub trait Sprite: Debug {
    fn update(&mut self) {}

    fn size(&self) -> Size {
        self.data().shape.size()
    }

    fn position(&self) -> Point {
        if self.data().rigid_handle.is_some() {
            return (
                self.rigid_body().translation().x,
                self.rigid_body().translation().y,
            )
                .into();
        }
        self.data().position
    }

    fn set_position(&mut self, pos: Point) {
        if self.data().collider_handle.is_some() {
            self.collider_mut().set_position([pos.x, pos.y].into());
        } else if self.data().rigid_handle.is_some() {
            self.rigid_body_mut().set_position([pos.x, pos.y].into(), true)
        }
        self.data_mut().position = pos;
    }

    fn rotation(&self) -> f32 {
        if self.data().rigid_handle.is_some() {
            return self.rigid_body().rotation().angle();
        }
        self.data().rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        if self.data().rigid_handle.is_some() {
            self.rigid_body_mut().set_rotation(rotation, true);
        } else {
            self.data_mut().rotation = rotation
        }
    }

    fn rigid_body(&self) -> &RigidBody {
        &self.level().rigid_bodies()[self.data().rigid_handle.unwrap()]
    }

    fn rigid_body_mut(&mut self) -> &mut RigidBody {
        let handle = self.data().rigid_handle.unwrap();
        &mut self.level_mut().rigid_bodies_mut()[handle]
    }

    fn collider(&self) -> &Collider {
        &self.level().colliders()[self.data().collider_handle.unwrap()]
    }

    fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.data().collider_handle.unwrap();
        &mut self.level_mut().colliders_mut()[handle]
    }

    fn contains(&self, point: Point) -> bool {
        let pos = self.position();
        let size = self.size();
        point.x >= pos.x - size.width
            && point.x <= pos.x + size.width
            && point.y >= pos.y - size.height
            && point.y <= pos.y + size.height
    }

    fn color(&self) -> Color {
        self.data().color
    }

    fn set_color(&mut self, color: Color) {
        self.data_mut().color = color
    }

    fn image(&self) -> Option<&Image> {
        self.data().image.as_ref()
    }

    fn image_mut(&mut self) -> Option<&mut Image> {
        self.data_mut().image.as_mut()
    }

    fn set_image(&mut self, image: Image) {
        self.data_mut().image = image.into()
    }

    fn is_selected(&self) -> bool {
        self.data().is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.data_mut().is_selected = selected
    }

    fn remove(&mut self) {
        let address = self.address();
        self.level_mut().remove(address);
    }

    fn level(&self) -> &Rglica<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &self.data().level
    }

    fn level_mut(&mut self) -> &mut Rglica<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &mut self.data_mut().level
    }

    fn draw(&self) {
        self.level().drawer().draw(self.data())
    }

    fn data(&self) -> &SpriteData;
    fn data_mut(&mut self) -> &mut SpriteData;
    fn make(shape: Shape, position: Point, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized;
}
