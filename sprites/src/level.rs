use std::borrow::Borrow;

use gm::Point;
use rapier2d::{na::Vector2, prelude::RigidBodySet};
use rtools::Rglica;

use crate::{Body, Collider, LevelBase, Sprite, SpriteBase, SpritesDrawer};
pub trait Level {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: String) {}

    fn gravity(&self) -> Point {
        let gravity = self.level().gravity.borrow();
        (gravity[0], gravity[1]).into()
    }

    fn set_gravity(&mut self, g: Point) {
        self.level_mut().gravity = Vector2::new(g.x, g.y)
    }

    fn player(&mut self) -> &mut Rglica<Body> {
        if self.level().player.is_null() {
            panic!("Getting null player from Level");
        }
        &mut self.level_mut().player
    }

    fn sprites(&self) -> &[Box<dyn Sprite>] {
        &self.level().sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.level().sets.rigid_body
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.level_mut().sets.rigid_body
    }

    fn add_body(&mut self, sprite: SpriteBase) -> Rglica<Body> {
        self.level_mut().add_body(sprite)
    }

    fn add_sprite(&mut self, sprite: SpriteBase) {
        self.level_mut().add_sprite(sprite)
    }

    fn remove_sprite(&mut self, address: u64) {
        self.level_mut().remove_sprite(address)
    }

    fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Collider> {
        self.level_mut().add_wall(sprite)
    }

    fn set_camera_rotation(&mut self, angle: f32) {
        self.drawer().set_camera_rotation(angle)
    }

    fn level(&self) -> &LevelBase;
    fn level_mut(&mut self) -> &mut LevelBase;
    fn drawer(&self) -> &dyn SpritesDrawer {
        todo!()
    }
}
