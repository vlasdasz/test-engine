use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gm::Point;
use rapier2d::{
    na::Vector2,
    prelude::{
        BroadPhase, CCDSolver, IntegrationParameters, IslandManager, JointSet, NarrowPhase, PhysicsPipeline,
    },
};
use rtools::{address::Address, Event, Rglica, Unwrap};

use crate::{sets::Sets, Level, Player, Sprite, SpritesDrawer};

pub struct LevelBase {
    pub player: Unwrap<Player>,
    pub drawer: Rglica<dyn SpritesDrawer>,

    pub cursor_position: Point,

    pub on_tap:             Event<Point>,
    pub on_sprite_selected: Event<Rglica<dyn Sprite>>,

    pub(crate) sprites: Vec<Box<dyn Sprite>>,
    pub(crate) sets:    Sets,
    pub(crate) gravity: Vector2<f32>,

    pub(crate) physics_pipeline: PhysicsPipeline,
    pub(crate) island_manager:   IslandManager,
    pub(crate) broad_phase:      BroadPhase,
    pub(crate) narrow_phase:     NarrowPhase,
    pub(crate) joint_set:        JointSet,
    pub(crate) ccd_solver:       CCDSolver,

    physics_hooks: (),
    event_handler: (),

    integration_parameters: IntegrationParameters,
}

impl LevelBase {
    pub fn update_physics(&mut self) {
        //  self.integration_parameters.dt = 0.5;

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
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub(crate) fn remove(&mut self, sprite: u64) {
        let index = self.sprites.iter().position(|a| a.address() == sprite).unwrap();

        let sprite = self.sprites[index].deref();

        if let Some(collider) = sprite.collider_handle() {
            self.sets.collider.remove(
                collider,
                &mut self.island_manager,
                &mut self.sets.rigid_body,
                true,
            );
        }

        if let Some(rigid_body) = sprite.rigid_body_handle() {
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

impl Level for LevelBase {
    fn level(&self) -> &LevelBase {
        self
    }

    fn level_mut(&mut self) -> &mut LevelBase {
        self
    }

    fn drawer(&self) -> &dyn SpritesDrawer {
        self.drawer.deref()
    }

    fn drawer_mut(&mut self) -> &mut dyn SpritesDrawer {
        self.drawer.deref_mut()
    }
}

impl Default for LevelBase {
    fn default() -> Self {
        Self {
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

            physics_hooks: Default::default(),
            event_handler: Default::default(),
            player:        Default::default(),

            integration_parameters: Default::default(),
        }
    }
}

impl Debug for LevelBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "LevelBase".fmt(f)
    }
}
