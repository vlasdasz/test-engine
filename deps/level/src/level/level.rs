use std::ops::{Deref, DerefMut};

use gm::{flat::Point, volume::GyroData};
use rapier2d::prelude::{ColliderSet, RigidBodySet};
use refs::{AsAny, Own, Weak};

use crate::{LevelBase, LevelManager, Sprite};

pub trait Level: AsAny + Deref<Target = LevelBase> + DerefMut {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: char) {}

    fn on_gyro_changed(&mut self, _: GyroData) {}

    fn update_camera(&mut self) {
        if let Some(player) = self.player.get() {
            *LevelManager::camera_pos() = player.position();
        }
    }

    // fn add_touch(&mut self, pos: Point) {
    //     let pos = self.convert_touch(pos);
    //     self.base_mut().on_tap.trigger(pos);
    // }

    // fn convert_touch(&self, pos: Point) -> Point {
    //     // let mut pos = pos;
    //     // let size = get_sprites_drawer().resolution();
    //     //
    //     // pos.x -= size.width.lossy_convert() / 2.;
    //     // pos.y -= size.height.lossy_convert() / 2.;
    //     // pos.y = -pos.y;
    //     // pos /= 10;
    //     //
    //     // pos *= 2;
    //     // pos /= get_sprites_drawer().scale();
    //     //
    //     // pos += get_sprites_drawer().camera_position();
    //     //
    //     // pos
    //     pos
    // }

    fn sprite_at(&self, point: Point) -> Option<Weak<dyn Sprite>> {
        for sprite in &self.sprites {
            if sprite.contains(point) {
                return sprite.weak().into();
            }
        }
        None
    }

    // fn scale(&self) -> f32 {
    //     //get_sprites_drawer().scale()
    //     1.0
    // }
    //
    // fn multiply_scale(&mut self, mul: f32) {
    //     let scale = self.scale() * mul;
    //     self.set_scale(scale);
    // }
    //
    // fn set_scale(&mut self, _scale: f32) {
    //     //get_sprites_drawer().set_scale(scale)
    // }

    fn gravity(&self) -> Point {
        let gravity = &self.gravity;
        (gravity[0], gravity[1]).into()
    }

    fn sprites(&self) -> &[Own<dyn Sprite>] {
        &self.sprites
    }

    fn sprites_mut(&mut self) -> &mut [Own<dyn Sprite>] {
        &mut self.sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.sets.rigid_body
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.sets.rigid_body
    }

    fn colliders(&self) -> &ColliderSet {
        &self.sets.collider
    }

    fn colliders_mut(&mut self) -> &mut ColliderSet {
        &mut self.sets.collider
    }
}
