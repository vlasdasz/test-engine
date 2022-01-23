use std::{ops::Deref, rc::Rc};

use rapier2d::{
    na::Vector2,
    prelude::{
        BroadPhase, CCDSolver, ColliderBuilder, IntegrationParameters, IslandManager, JointSet,
        NarrowPhase, PhysicsPipeline, RigidBodyBuilder,
    },
};
use rtools::{Rglica, ToRglica};

use crate::{
    sets::Sets, sprites_drawer::DummyDrawer, Body, Collider, Level, Sprite, SpriteBase,
    SpritesDrawer,
};

pub struct LevelBase {
    pub player:  Rglica<Body>,
    pub sprites: Vec<Box<dyn Sprite>>,
    pub drawer:  Rc<dyn SpritesDrawer>,

    pub(crate) sets: Sets,

    pub(crate) gravity: Vector2<f32>,
    physics_pipeline:   PhysicsPipeline,
    island_manager:     IslandManager,
    broad_phase:        BroadPhase,
    narrow_phase:       NarrowPhase,
    joint_set:          JointSet,
    ccd_solver:         CCDSolver,

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

    pub fn add_body(&mut self, sprite: SpriteBase) -> Rglica<Body> {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(sprite.position().x, sprite.position().y))
            .build();
        let collider = ColliderBuilder::cuboid(sprite.size().width, sprite.size().height)
            .restitution(0.7)
            .build();

        let body_handle = self.sets.rigid_body.insert(rigid_body);
        self.sets
            .collider
            .insert_with_parent(collider, body_handle, &mut self.sets.rigid_body);
        let boxed = Box::new(Body::make(sprite, body_handle, self));
        let body = boxed.to_rglica();
        self.sprites.push(boxed);
        body
    }

    pub fn add_sprite(&mut self, sprite: SpriteBase) {
        self.sprites.push(Box::new(sprite))
    }

    pub fn remove_sprite(&mut self, _address: u64) {
        //self.sp
    }

    pub fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Collider> {
        let collider = ColliderBuilder::cuboid(sprite.size().width, sprite.size().height)
            .translation(Vector2::new(sprite.position().x, sprite.position().y))
            .build();
        self.sets.collider.insert(collider);
        let boxed = Box::<Collider>::new(sprite.into());
        let wall = boxed.to_rglica();
        self.sprites.push(boxed);
        wall
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
