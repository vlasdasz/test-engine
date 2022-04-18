use std::{fmt::Debug, ops::Deref};

use gm::flat::{Point, Shape};
use rapier2d::{
    geometry::ContactEvent,
    na::Vector2,
    parry::partitioning::IndexedData,
    prelude::{
        BroadPhase, CCDSolver, ChannelEventCollector, IntegrationParameters, IslandManager, JointSet,
        NarrowPhase, PhysicsPipeline,
    },
};
use rtools::{address::Address, Event, Rglica, ToRglica};

use crate::{sets::Sets, Level, Sprite, SpritesDrawer};

pub struct LevelBase {
    pub drawer: Rglica<dyn SpritesDrawer>,

    pub cursor_position: Point,

    pub on_tap:             Event<Point>,
    pub on_sprite_selected: Event<Rglica<dyn Sprite>>,

    pub(crate) colliding_sprites: Vec<Rglica<dyn Sprite>>,

    pub(crate) sprites: Vec<Box<dyn Sprite>>,
    pub(crate) sets:    Sets,
    pub(crate) gravity: Vector2<f32>,

    pub(crate) physics_pipeline: PhysicsPipeline,
    pub(crate) island_manager:   IslandManager,
    pub(crate) broad_phase:      BroadPhase,
    pub(crate) narrow_phase:     NarrowPhase,
    pub(crate) joint_set:        JointSet,
    pub(crate) ccd_solver:       CCDSolver,

    integration_parameters: IntegrationParameters,
}

impl LevelBase {
    pub fn update_physics(&mut self) {
        //  self.integration_parameters.dt = 0.5;

        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, _intersection_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(intersection_send, contact_send);

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.sets.rigid_body,
            &mut self.sets.collider,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &(),
            &event_handler,
        );

        while let Ok(contact_event) = contact_recv.try_recv() {
            if let ContactEvent::Started(a, b) = contact_event {
                for sprite in &self.colliding_sprites {
                    let handle = sprite.data().collider_handle.unwrap();

                    let other_index = if b.index() == handle.index() {
                        a
                    } else if a.index() == handle.index() {
                        b
                    } else {
                        panic!()
                    };

                    if let Some(other) = self.sprite_with_index(other_index.index()) {
                        sprite.to_rglica().data_mut().on_collision.trigger(other);
                    }
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
            .map(|a| a.to_rglica())
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
                &mut self.joint_set,
            );
        }

        self.sprites.remove(index);
    }
}

impl Default for LevelBase {
    fn default() -> Self {
        Self {
            colliding_sprites: Default::default(),

            sprites: Default::default(),
            drawer:  Default::default(),

            cursor_position: Default::default(),

            on_tap:             Default::default(),
            on_sprite_selected: Default::default(),

            sets:             Default::default(),
            gravity:          Vector2::new(0.0, -9.81),
            physics_pipeline: Default::default(),
            island_manager:   IslandManager::new(),
            broad_phase:      BroadPhase::new(),
            narrow_phase:     NarrowPhase::new(),
            joint_set:        JointSet::new(),
            ccd_solver:       CCDSolver::new(),

            integration_parameters: Default::default(),
        }
    }
}

impl Debug for LevelBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "LevelBase".fmt(f)
    }
}

pub fn add_sprite<T: 'static + Sprite>(
    shape: impl Into<Shape>,
    position: impl Into<Point>,
    level: &mut dyn Level,
) -> Rglica<T> {
    let sprite = T::make(shape.into(), position.into(), level.rglica());
    let result = sprite.to_rglica();
    level.add_sprite(sprite);
    result
}
