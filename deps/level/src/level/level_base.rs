use std::{collections::HashMap, ops::Deref};

use gm::flat::Point;
use rapier2d::{
    dynamics::ImpulseJointSet,
    geometry::ColliderHandle,
    na::Vector2,
    prelude::{
        BroadPhaseMultiSap, CCDSolver, CollisionEvent, IntegrationParameters, IslandManager,
        MultibodyJointSet, NarrowPhase, PhysicsPipeline,
    },
};
use refs::{Own, Weak};
use smart_default::SmartDefault;
use vents::Event;
use wgpu_wrapper::image::Image;

use crate::{event_handler::EventHandler, sets::Sets, Level, Player, Sprite};

#[derive(SmartDefault)]
pub struct LevelBase {
    pub player: Weak<Player>,

    pub background: Weak<Image>,

    pub cursor_position: Point,

    pub on_tap:             Event<Point>,
    pub on_sprite_selected: Event<Weak<dyn Sprite>>,

    pub(crate) colliding_sprites: HashMap<ColliderHandle, Weak<dyn Sprite>>,

    pub(crate) sprites: Vec<Own<dyn Sprite>>,
    pub(crate) sets:    Sets,

    #[default(Vector2::new(0.0, -9.81))]
    pub(crate) gravity: Vector2<f32>,

    pub(crate) physics_pipeline: PhysicsPipeline,

    pub(crate) island_manager:   IslandManager,
    pub(crate) broad_phase:      BroadPhaseMultiSap,
    pub(crate) narrow_phase:     NarrowPhase,
    pub(crate) impulse_joints:   ImpulseJointSet,
    pub(crate) multibody_joints: MultibodyJointSet,
    pub(crate) ccd_solver:       CCDSolver,

    pub(crate) events: EventHandler,

    integration_parameters: IntegrationParameters,
}

impl LevelBase {
    pub fn update_physics(&mut self, frame_time: f32) {
        self.integration_parameters.dt = frame_time;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.sets.rigid_bodies,
            &mut self.sets.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &(),
            &self.events.handler,
        );

        self.handle_collisions();
    }

    fn handle_collisions(&self) {
        while let Ok(contact) = self.events.intersection.try_recv() {
            let CollisionEvent::Started(a, b, _) = contact else {
                continue;
            };

            let a = self
                .colliding_sprites
                .get(&a)
                .copied()
                .unwrap_or_else(|| self.sprite_with_collider(a).unwrap());

            let b = self
                .colliding_sprites
                .get(&b)
                .copied()
                .unwrap_or_else(|| self.sprite_with_collider(b).unwrap());

            if a.collision_enabled {
                a.on_collision.trigger(b);
            }

            if b.collision_enabled {
                b.on_collision.trigger(a);
            }
        }
    }

    fn sprite_with_collider(&self, collider_handle: ColliderHandle) -> Option<Weak<dyn Sprite>> {
        self.sprites
            .iter()
            .find(|a| match a.collider_handle() {
                Some(handle) => handle == collider_handle,
                None => false,
            })
            .map(Own::weak)
    }

    pub(crate) fn remove(&mut self, sprite: usize) {
        let index = self.sprites.iter().position(|a| a.addr() == sprite).unwrap();

        let sprite = self.sprites[index].deref();

        if let Some(collider) = sprite.collider_handle() {
            self.sets.colliders.remove(
                collider,
                &mut self.island_manager,
                &mut self.sets.rigid_bodies,
                true,
            );
        }

        if let Some(rigid_body) = sprite.rigid_handle() {
            self.sets.rigid_bodies.remove(
                rigid_body,
                &mut self.island_manager,
                &mut self.sets.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                true,
            );
        }

        self.sprites.remove(index);
    }
}

pub trait LevelTemplates {
    fn set_gravity(&mut self, g: impl Into<Point>);
}

impl<T: ?Sized + Level> LevelTemplates for T {
    fn set_gravity(&mut self, g: impl Into<Point>) {
        let g = g.into();
        self.gravity = Vector2::new(g.x, g.y);
    }
}
