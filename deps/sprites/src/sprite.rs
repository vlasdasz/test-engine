use gl_image::Image;
use gm::{
    flat::{Point, Shape, Size},
    Color,
};
use rapier2d::{geometry::Collider, prelude::RigidBody};
use refs::{Strong, Weak};
use rtools::{address::Address, data_manager::Handle, IntoF32};

use crate::{get_sprites_drawer, Level, SpriteData};

pub trait Sprite {
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

    fn rotation(&self) -> f32 {
        if self.data().rigid_handle.is_some() {
            return self.rigid_body().rotation().angle();
        }
        if self.data().collider_handle.is_some() {
            return self.collider().rotation().angle();
        }
        self.data().rotation
    }

    fn restitution(&mut self) -> f32 {
        self.collider().restitution()
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

    fn color(&self) -> &Color {
        &self.data().color
    }

    fn image(&self) -> Handle<Image> {
        self.data().image
    }

    fn is_selected(&self) -> bool {
        self.data().is_selected
    }

    fn remove(&mut self) {
        let address = self.address();
        self.level_mut().remove(address);
    }

    fn level(&self) -> &Weak<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &self.data().level
    }

    fn level_mut(&mut self) -> &mut Weak<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &mut self.data_mut().level
    }

    fn draw(&self) {
        get_sprites_drawer().draw(self.data())
    }

    fn data(&self) -> &SpriteData;
    fn data_mut(&mut self) -> &mut SpriteData;
    fn make(shape: Shape, position: Point, level: Weak<dyn Level>) -> Strong<Self>
    where
        Self: Sized;
}

pub trait SpriteTemplates {
    fn set_color(&mut self, _: Color) -> &mut Self;
    fn set_selected(&mut self, _: bool) -> &mut Self;
    fn set_image(&mut self, _: Handle<Image>) -> &mut Self;
    fn set_restitution(&mut self, _: f32) -> &mut Self;
    fn set_position(&mut self, _: Point) -> &mut Self;
    fn set_rotation(&mut self, _: impl IntoF32) -> &mut Self;
}

impl<T: ?Sized + Sprite> SpriteTemplates for T {
    fn set_color(&mut self, color: Color) -> &mut Self {
        self.data_mut().color = color;
        self
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.data_mut().is_selected = selected;
        self
    }

    fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.data_mut().image = image;
        self
    }

    fn set_restitution(&mut self, res: f32) -> &mut Self {
        self.collider_mut().set_restitution(res);
        self
    }

    fn set_position(&mut self, pos: Point) -> &mut Self {
        if self.data().collider_handle.is_some() {
            self.collider_mut().set_position([pos.x, pos.y].into());
        } else if self.data().rigid_handle.is_some() {
            self.rigid_body_mut().set_position([pos.x, pos.y].into(), true)
        }
        self.data_mut().position = pos;
        self
    }

    fn set_rotation(&mut self, rotation: impl IntoF32) -> &mut Self {
        let rotation = rotation.into_f32();
        if self.data().rigid_handle.is_some() {
            self.rigid_body_mut().set_rotation(rotation, true);
        }
        if self.data().collider_handle.is_some() {
            self.collider_mut().set_rotation(rotation);
        } else {
            self.data_mut().rotation = rotation
        }
        self
    }
}
