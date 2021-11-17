use std::{default::default, ops::Deref, rc::Rc};

use gm::Point;
use rapier2d::{
    na::Vector2,
    prelude::{
        BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager,
        JointSet, NarrowPhase, PhysicsPipeline, RigidBodyBuilder, RigidBodySet,
    },
};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use tools::{Rglica, ToRglica};

use crate::{sprites_drawer::DummyDrawer, Body, Collider, Sprite, SpriteBase, SpritesDrawer};

pub trait Control {
    fn jump(&mut self);
    fn go_left(&mut self);
    fn go_right(&mut self);
    fn add_impulse(&mut self, impulse: &Point);
}

pub trait Level {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn set_gravity(&mut self, g: Point) {
        self.level_mut().gravity = Vector2::new(g.x, g.y)
    }

    fn player(&mut self) -> &mut Rglica<Body> {
        &mut self.level_mut().player
    }

    fn sprites(&self) -> &[Box<dyn Sprite>] {
        &self.level().sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.level().rigid_body_set
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.level_mut().rigid_body_set
    }

    fn add_body(&mut self, sprite: SpriteBase) -> Rglica<Body> {
        self.level_mut().add_body(sprite)
    }

    fn add_sprite(&mut self, sprite: SpriteBase) {
        self.level_mut().add_sprite(sprite)
    }

    fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Collider> {
        self.level_mut().add_wall(sprite)
    }

    fn set_camera_rotation(&mut self, angle: f32) {
        self.drawer().set_camera_rotation(angle)
    }

    fn on_key_pressed(&mut self, _key: String) {}

    fn level(&self) -> &LevelBase;
    fn level_mut(&mut self) -> &mut LevelBase;
    fn drawer(&self) -> &dyn SpritesDrawer {
        todo!()
    }
}

pub struct LevelBase {
    pub player:  Rglica<Body>,
    pub sprites: Vec<Box<dyn Sprite>>,
    pub drawer:  Rc<dyn SpritesDrawer>,

    rigid_body_set: RigidBodySet,
    collider_set:   ColliderSet,

    gravity:          Vector2<f32>,
    physics_pipeline: PhysicsPipeline,
    island_manager:   IslandManager,
    broad_phase:      BroadPhase,
    narrow_phase:     NarrowPhase,
    joint_set:        JointSet,
    ccd_solver:       CCDSolver,

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
            &mut self.rigid_body_set,
            &mut self.collider_set,
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
        let body_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        let boxed = Box::new(Body::make(sprite, body_handle, self));
        let body = boxed.to_rglica();
        self.sprites.push(boxed);
        body
    }

    pub fn add_sprite(&mut self, sprite: SpriteBase) {
        self.sprites.push(Box::new(sprite))
    }

    pub fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Collider> {
        let collider = ColliderBuilder::cuboid(sprite.size().width, sprite.size().height)
            .translation(Vector2::new(sprite.position().x, sprite.position().y))
            .build();
        self.collider_set.insert(collider);
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
            sprites:        vec![],
            rigid_body_set: RigidBodySet::new(),
            drawer:         Rc::new(DummyDrawer::default()),

            collider_set:     ColliderSet::new(),
            gravity:          Vector2::new(0.0, -9.81),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager:   IslandManager::new(),
            broad_phase:      BroadPhase::new(),
            narrow_phase:     NarrowPhase::new(),
            joint_set:        JointSet::new(),
            ccd_solver:       CCDSolver::new(),

            physics_hooks: (),
            event_handler: (),
            player:        default(),

            integration_parameters: IntegrationParameters::default(),
        }
    }
}

impl Serialize for LevelBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Level", 2)?;
        s.serialize_field("sprites", &self.sprites)?;
        s.serialize_field("player", self.player.deref())?;
        s.end()
    }
}

impl<'a> Deserialize<'a> for LevelBase {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        todo!()
    }
}
