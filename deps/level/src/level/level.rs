use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use refs::{AsAny, Own, Weak};
use wgpu_wrapper::WGPUApp;

use super::LevelInternal;
use crate::{LevelBase, LevelManager, Sprite};

pub trait Level: AsAny + Deref<Target = LevelBase> + DerefMut + LevelInternal {
    fn update_camera(&mut self) {
        if let Some(player) = self.player.get() {
            *LevelManager::camera_pos() = player.position();
        }
    }

    fn add_touch(&mut self, pos: Point) -> bool {
        let pos = self.convert_touch(pos);
        dbg!(pos);
        true
    }

    fn convert_touch(&self, pos: Point) -> Point {
        let mut pos = pos;
        let size = WGPUApp::current().window_size;

        pos.x -= size.width / 2.0;
        pos.y -= size.height / 2.0;
        pos.y = -pos.y;
        pos /= 10;

        pos *= 2;
        pos /= WGPUApp::screen_scale();

        pos /= *LevelManager::scale();

        pos += *LevelManager::camera_pos();

        pos
    }

    fn sprite_at(&self, point: Point) -> Option<Weak<dyn Sprite>> {
        for sprite in &self.sprites {
            if sprite.contains(point) {
                return sprite.weak().into();
            }
        }
        None
    }

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
}
