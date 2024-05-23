use gm::{flat::Point, volume::GyroData};
use rapier2d::prelude::{ColliderSet, RigidBodySet};
use refs::{AsAny, Own, Weak};

use crate::{LevelBase, LevelManager, Player, Sprite};

pub trait Level: AsAny {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: char) {}

    fn on_gyro_changed(&mut self, _: GyroData) {}

    fn update_camera(&mut self) {
        if let Some(player) = self.player().get() {
            *LevelManager::camera_pos() = player.position();
        }
    }

    fn cursor_position(&self) -> Point {
        self.base().cursor_position
    }

    fn set_cursor_position(&mut self, pos: Point) {
        self.base_mut().cursor_position = self.convert_touch(pos)
    }

    fn add_touch(&mut self, pos: Point) {
        let pos = self.convert_touch(pos);
        self.base_mut().on_tap.trigger(pos);
    }

    fn convert_touch(&self, pos: Point) -> Point {
        // let mut pos = pos;
        // let size = get_sprites_drawer().resolution();
        //
        // pos.x -= size.width.lossy_convert() / 2.;
        // pos.y -= size.height.lossy_convert() / 2.;
        // pos.y = -pos.y;
        // pos /= 10;
        //
        // pos *= 2;
        // pos /= get_sprites_drawer().scale();
        //
        // pos += get_sprites_drawer().camera_position();
        //
        // pos
        pos
    }

    fn sprite_at(&self, point: Point) -> Option<Weak<dyn Sprite>> {
        for sprite in self.sprites() {
            if sprite.contains(point) {
                return sprite.weak().into();
            }
        }
        None
    }

    fn scale(&self) -> f32 {
        //get_sprites_drawer().scale()
        1.0
    }

    fn multiply_scale(&mut self, mul: f32) {
        let scale = self.scale() * mul;
        self.set_scale(scale);
    }

    fn set_scale(&mut self, _scale: f32) {
        //get_sprites_drawer().set_scale(scale)
    }

    fn gravity(&self) -> Point {
        let gravity = &self.base().gravity;
        (gravity[0], gravity[1]).into()
    }

    fn sprites(&self) -> &[Own<dyn Sprite>] {
        &self.base().sprites
    }

    fn sprites_mut(&mut self) -> &mut [Own<dyn Sprite>] {
        &mut self.base_mut().sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.base().sets.rigid_body
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.base_mut().sets.rigid_body
    }

    fn colliders(&self) -> &ColliderSet {
        &self.base().sets.collider
    }

    fn colliders_mut(&mut self) -> &mut ColliderSet {
        &mut self.base_mut().sets.collider
    }

    fn remove(&mut self, sprite: usize) {
        self.base_mut().remove(sprite)
    }

    fn player(&self) -> Weak<Player> {
        self.base().player
    }

    fn base(&self) -> &LevelBase;
    fn base_mut(&mut self) -> &mut LevelBase;
    fn weak_level(&self) -> Weak<dyn Level>;
}
