use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{Color, Point, Size};
use rapier2d::{geometry::ColliderHandle, prelude::RigidBodyHandle};
use rtools::address::Address;

use crate::{Level, SpriteBase};

pub trait Sprite: Debug {
    fn update(&mut self) {}

    fn size(&self) -> Size {
        self.sprite().size
    }

    fn position(&self) -> Point {
        self.sprite().position
    }

    fn rotation(&self) -> f32 {
        self.sprite().rotation
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
        self.sprite().color
    }

    fn set_color(&mut self, color: Color) {
        self.sprite_mut().color = color
    }

    fn image(&self) -> Option<&Image> {
        self.sprite().image.as_ref()
    }

    fn image_mut(&mut self) -> Option<&mut Image> {
        self.sprite_mut().image.as_mut()
    }

    fn set_image(&mut self, image: Image) {
        self.sprite_mut().image = image.into()
    }

    fn rigid_body_handle(&self) -> Option<RigidBodyHandle> {
        None
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        None
    }

    fn is_selected(&self) -> bool {
        self.sprite().is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.sprite_mut().is_selected = selected
    }

    fn remove(&mut self) {
        let address = self.address();
        self.level_mut().remove(address);
    }

    fn level(&self) -> &dyn Level {
        debug_assert!(self.sprite().level.is_ok(), "Null Level");
        self.sprite().level.deref()
    }

    fn level_mut(&mut self) -> &mut dyn Level {
        debug_assert!(self.sprite().level.is_ok(), "Null Level");
        self.sprite_mut().level.deref_mut()
    }

    fn draw(&self) {
        self.level().drawer().draw(self.sprite())
    }

    fn sprite(&self) -> &SpriteBase;
    fn sprite_mut(&mut self) -> &mut SpriteBase;
}
