use gm::{flat::Point, volume::GyroData, LossyConvert};
use rapier2d::prelude::{ColliderSet, RigidBodySet};
use refs::{Own, Weak};

use crate::{get_sprites_drawer, LevelBase, Player, Sprite};

pub trait Level {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: char) {}

    fn on_gyro_changed(&mut self, _: GyroData) {}

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
        let mut pos = pos;
        let size = get_sprites_drawer().resolution();

        pos.x -= size.width.lossy_convert() / 2.0;
        pos.y -= size.height.lossy_convert() / 2.0;
        pos.y = -pos.y;
        pos /= 10;

        pos *= 2;
        pos /= get_sprites_drawer().scale();

        pos += get_sprites_drawer().camera_position();

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
        get_sprites_drawer().scale()
    }

    fn multiply_scale(&mut self, mul: f32) {
        let scale = self.scale() * mul;
        self.set_scale(scale);
    }

    fn set_scale(&mut self, scale: f32) {
        get_sprites_drawer().set_scale(scale)
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

    fn set_camera_position(&mut self, pos: Point) {
        get_sprites_drawer().set_camera_position(pos)
    }

    fn set_camera_rotation(&mut self, angle: f32) {
        get_sprites_drawer().set_camera_rotation(angle)
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
