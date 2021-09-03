use rapier2d::na::Vector2;
use tools::Boxed;
use tools::New;
use tools::Rglica;
use tools::ToRglica;

use crate::{Body, Collider, Sprite, SpriteBase};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::Point;
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager,
    JointSet, NarrowPhase, PhysicsPipeline, RigidBodyBuilder, RigidBodySet,
};

pub trait Control {
    fn jump(&mut self);
    fn go_left(&mut self);
    fn go_right(&mut self);
    fn add_impulse(&mut self, impulse: &Point);
}

pub trait Level {
    fn level(&self) -> &LevelBase;
    fn level_mut(&mut self) -> &mut LevelBase;
    fn rigid_bodies(&self) -> &RigidBodySet;
    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet;
}

pub struct LevelBase {
    pub player: Rglica<Body>,
    pub sprites: Vec<Box<dyn Sprite>>,

    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,

    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl LevelBase {
    pub fn setup(&mut self) {
        self.player = self.add_body((0, 10, 17.0 / 6.0, 28.0 / 6.0).into());
        self.player.lock_rotations();
    }

    pub fn update(&mut self) {
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

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn on_key_pressed(&mut self, _key: Key, _action: Action) {
        // if key == Key::A {
        //     self.go_left()
        // } else if key == Key::D {
        //     self.go_right();
        // } else if key == Key::W {
        //     self.jump()
        // } else if key == Key::S {
        // }
    }

    // fn player_body(&mut self) -> &mut RigidBody {
    //     &mut self.rigid_body_set[self.player.borrow().rigid_body_handle.unwrap()]
    // }

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

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.rigid_body_set
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.rigid_body_set
    }
}

impl Boxed for LevelBase {
    fn boxed() -> Box<Self> {
        Box::new(Self {
            sprites: vec![],
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            gravity: Vector2::new(0.0, -9.81),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            player: Rglica::new(),
        })
    }
}
