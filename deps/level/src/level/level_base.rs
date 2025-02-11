use std::ops::Deref;

use educe::Educe;
use gm::flat::Point;
use rapier2d::na::Vector2;
use refs::{Own, Weak};
use vents::Event;
use window::image::Image;

use crate::{Level, LevelManager, Sprite, level::level_physics::LevelPhysics};

#[derive(Educe)]
#[educe(Default)]
pub struct LevelBase {
    pub(crate) sprites: Vec<Own<dyn Sprite>>,

    pub background: Weak<Image>,

    pub cursor_position: Point,

    pub on_tap:             Event<Point>,
    pub on_sprite_selected: Event<Weak<dyn Sprite>>,

    #[educe(Default = LevelManager::default_z_position())]
    pub(crate) last_z_pos: f32,

    pub(crate) physics: Option<LevelPhysics>,
}

impl LevelBase {
    pub fn init_physics(&mut self) {
        assert!(self.physics.is_none(), "Double init_physics");
        self.physics = LevelPhysics::default().into();
    }

    pub fn update_physics(&mut self, frame_time: f32) {
        if let Some(physics) = self.physics.as_mut() {
            physics.update_physics(&self.sprites, frame_time);
        }
    }

    pub(crate) fn remove(&mut self, sprite: usize) {
        let index = self.sprites.iter().position(|a| a.addr() == sprite).unwrap();

        let sprite = self.sprites[index].deref();

        if let Some(physics) = self.physics.as_mut() {
            physics.remove(sprite);
        }
        self.sprites.remove(index);
    }

    pub fn remove_all_sprites(&mut self) {
        if let Some(physics) = &mut self.physics {
            for sprite in self.sprites.drain(..) {
                physics.remove(sprite.deref());
            }
        } else {
            self.sprites.clear();
        }
    }
}

pub trait LevelTemplates {
    fn set_gravity(&mut self, g: impl Into<Point>);
}

impl<T: ?Sized + Level> LevelTemplates for T {
    fn set_gravity(&mut self, g: impl Into<Point>) {
        let g = g.into();
        if let Some(physics) = self.physics.as_mut() {
            physics.gravity = Vector2::new(g.x, g.y);
        }
    }
}
