use std::{fmt::Debug, ops::Deref, rc::Rc};

use rapier2d::{
    na::Vector2,
    prelude::{
        BroadPhase, CCDSolver, IntegrationParameters, IslandManager, JointSet, NarrowPhase,
        PhysicsPipeline,
    },
};
use rtools::Rglica;

use crate::{sets::Sets, sprites_drawer::DummyDrawer, Body, Level, Sprite, SpritesDrawer};

pub struct LevelBase {
    pub player: Rglica<Body>,
    pub drawer: Rc<dyn SpritesDrawer>,

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

    pub(crate) fn _remove(&mut self, _sprite: &dyn Sprite) {}
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
}

impl Default for LevelBase {
    fn default() -> Self {
        Self {
            sprites: vec![],
            drawer:  Rc::new(DummyDrawer::default()),

            sets:             Sets::default(),
            gravity:          Vector2::new(0.0, -9.81),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager:   IslandManager::new(),
            broad_phase:      BroadPhase::new(),
            narrow_phase:     NarrowPhase::new(),
            joint_set:        JointSet::new(),
            ccd_solver:       CCDSolver::new(),

            physics_hooks: (),
            event_handler: (),
            player:        Default::default(),

            integration_parameters: IntegrationParameters::default(),
        }
    }
}

impl Debug for LevelBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "LevelBase".fmt(f)
    }
}
