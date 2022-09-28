use std::ops::Deref;

use gm::flat::Point;
use rapier2d::{
    dynamics::ImpulseJointSet,
    na::Vector2,
    parry::partitioning::IndexedData,
    prelude::{
        BroadPhase, CCDSolver, CollisionEvent, IntegrationParameters, IslandManager, MultibodyJointSet,
        NarrowPhase, PhysicsPipeline,
    },
};
use rtools::{address::Address, Event, Rglica, ToWeak};
use smart_default::SmartDefault;

use crate::{event_handler::EventHandler, sets::Sets, Level, Player, Sprite};

#[derive(SmartDefault)]
pub struct LevelBase {
    pub player: Rglica<Player>,

    pub cursor_position: Point,

    pub on_tap:             Event<Point>,
    pub on_sprite_selected: Event<Rglica<dyn Sprite>>,

    pub(crate) colliding_sprites: Vec<Rglica<dyn Sprite>>,

    pub(crate) sprites: Vec<Box<dyn Sprite>>,
    pub(crate) sets:    Sets,

    #[default(Vector2::new(0.0, -9.81))]
    pub(crate) gravity: Vector2<f32>,

    pub(crate) physics_pipeline: PhysicsPipeline,

    pub(crate) island_manager:   IslandManager,
    #[default(BroadPhase::new())]
    pub(crate) broad_phase:      BroadPhase,
    #[default(NarrowPhase::new())]
    pub(crate) narrow_phase:     NarrowPhase,
    pub(crate) impulse_joints:   ImpulseJointSet,
    pub(crate) multibody_joints: MultibodyJointSet,
    #[default(CCDSolver::new())]
    pub(crate) ccd_solver:       CCDSolver,

    pub(crate) events: EventHandler,

    integration_parameters: IntegrationParameters,
}

impl LevelBase {
    pub fn update_physics(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.sets.rigid_body,
            &mut self.sets.collider,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &self.events.handler,
        );

        self.handle_collisions();
    }

    fn handle_collisions(&self) {
        while let Ok(contact) = self.events.intersection.try_recv() {
            let (a, b) = match contact {
                CollisionEvent::Started(a, b, _) => (a, b),
                _ => continue,
            };

            for sprite in &self.colliding_sprites {
                let handle = sprite.data().collider_handle.unwrap();

                let other_index = if b == handle {
                    a
                } else if a == handle {
                    b
                } else {
                    panic!()
                };

                if let Some(other) = self.sprite_with_index(other_index.index()) {
                    sprite.weak().data_mut().on_collision.trigger(other);
                }
            }
        }
    }

    fn sprite_with_index(&self, index: usize) -> Option<Rglica<dyn Sprite>> {
        self.sprites
            .iter()
            .find(|a| match a.data().collider_handle {
                Some(handle) => handle.index() == index,
                None => false,
            })
            .map(|a| a.weak())
    }

    pub(crate) fn remove(&mut self, sprite: u64) {
        let index = self.sprites.iter().position(|a| a.address() == sprite).unwrap();

        let sprite = self.sprites[index].deref();

        if let Some(collider) = sprite.data().collider_handle {
            self.sets.collider.remove(
                collider,
                &mut self.island_manager,
                &mut self.sets.rigid_body,
                true,
            );
        }

        if let Some(rigid_body) = sprite.data().rigid_handle {
            self.sets.rigid_body.remove(
                rigid_body,
                &mut self.island_manager,
                &mut self.sets.collider,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                true,
            );
        }

        self.sprites.remove(index);
    }
}

impl Level for LevelBase {
    fn base(&self) -> &LevelBase {
        self
    }
    fn base_mut(&mut self) -> &mut LevelBase {
        self
    }
    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).weak()
    }
}

pub trait LevelTemplates {
    fn set_gravity(&mut self, g: impl Into<Point>);
}

impl<T: ?Sized + Level> LevelTemplates for T {
    fn set_gravity(&mut self, g: impl Into<Point>) {
        let g = g.into();
        self.base_mut().gravity = Vector2::new(g.x, g.y)
    }
}
