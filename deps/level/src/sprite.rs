use gm::{
    flat::{Point, Shape, Size},
    Color, ToF32,
};
use rapier2d::{
    geometry::Collider,
    prelude::{CoefficientCombineRule, RigidBody, Rotation},
};
use refs::{Address, Own, Weak};
use wgpu_wrapper::image::{Image, ToImage};

use crate::{Level, LevelManager, SpriteData};

pub trait Sprite {
    fn update(&mut self) {}

    fn size(&self) -> Size {
        self.data().shape.size()
    }

    fn position(&self) -> Point {
        if self.data().rigid_handle.is_some() {
            let rigid_body = self.rigid_body();
            return (rigid_body.translation().x, rigid_body.translation().y).into();
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
        &LevelManager::level().rigid_bodies()[self.data().rigid_handle.unwrap()]
    }

    fn rigid_body_mut(&mut self) -> &mut RigidBody {
        let handle = self.data().rigid_handle.unwrap();
        &mut LevelManager::level_mut().rigid_bodies_mut()[handle]
    }

    fn collider(&self) -> &Collider {
        &LevelManager::level().colliders()[self.data().collider_handle.unwrap()]
    }

    fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.data().collider_handle.unwrap();
        &mut LevelManager::level_mut().colliders_mut()[handle]
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

    fn image(&self) -> Weak<Image> {
        self.data().image
    }

    fn is_selected(&self) -> bool {
        self.data().is_selected
    }

    fn remove(&mut self) {
        let address = self.address();
        LevelManager::level_mut().remove(address);
    }

    fn data(&self) -> &SpriteData;
    fn data_mut(&mut self) -> &mut SpriteData;
    fn make(shape: Shape, position: Point, level: Weak<dyn Level>) -> Own<Self>
    where Self: Sized;
}

pub trait SpriteTemplates {
    fn set_color(&mut self, _: Color) -> &mut Self;
    fn set_selected(&mut self, _: bool) -> &mut Self;
    fn set_image(&mut self, _: impl ToImage) -> &mut Self;
    fn set_restitution(&mut self, _: f32, _: CoefficientCombineRule) -> &mut Self;
    fn set_position(&mut self, _: Point) -> &mut Self;
    fn set_rotation(&mut self, _: impl ToF32) -> &mut Self;
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

    fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.data_mut().image = image.to_image();
        self
    }

    fn set_restitution(&mut self, res: f32, rule: CoefficientCombineRule) -> &mut Self {
        self.collider_mut().set_restitution(res);
        self.collider_mut().set_restitution_combine_rule(rule);
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

    fn set_rotation(&mut self, rotation: impl ToF32) -> &mut Self {
        let rotation = rotation.to_f32();
        if self.data().rigid_handle.is_some() {
            self.rigid_body_mut().set_rotation(Rotation::new(rotation), true);
        }
        if self.data().collider_handle.is_some() {
            self.collider_mut().set_rotation(Rotation::new(rotation));
        } else {
            self.data_mut().rotation = rotation
        }
        self
    }
}
