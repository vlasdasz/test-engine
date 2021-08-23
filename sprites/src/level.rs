use rapier2d::na::Vector2;
use tools::refs::{make_shared, new_shared, Shared};
use tools::New;

use crate::{Collider, Sprite, SpriteBase};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::Point;
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager,
    JointSet, NarrowPhase, PhysicsPipeline, RigidBody, RigidBodyBuilder, RigidBodySet,
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
}

pub struct LevelBase {
    pub sprites: Vec<Shared<dyn Sprite>>,

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
        // let player = self.add_rect((0, 10).into(), (17.0 / 6.0, 28.0 / 6.0).into());
        // self.player = player;
        // // self.player.borrow_mut().image =
        // //     Some(Image::load(&crate::te::paths::images().join("frisk.png")));
        // let body = self.player_body();
        // body.lock_rotations(true, true);
        // dbg!(body.mass());
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

        // for sprite in &self.sprites {
        //     let mut sprite = sprite.borrow_mut();
        //
        //     let body = &self.rigid_body_set[sprite.rigid_body_handle.unwrap()];
        //
        //     sprite.position.x = body.translation().x;
        //     sprite.position.y = body.translation().y;
        //     sprite.rotation = body.rotation().angle();
        // }
    }

    // pub fn add_rect(&mut self, pos: gm::Point, size: gm::Size) -> Shared<SpriteBase> {
    //     let rigid_body = RigidBodyBuilder::new_dynamic()
    //         .translation(Vector2::new(pos.x, pos.y))
    //         .build();
    //     let collider = ColliderBuilder::cuboid(size.width, size.height)
    //         .restitution(0.7)
    //         .build();
    //     let ball_body_handle = self.rigid_body_set.insert(rigid_body);
    //     let handle = self.collider_set.insert_with_parent(
    //         collider,
    //         ball_body_handle,
    //         &mut self.rigid_body_set,
    //     );
    //     let sprite = SpriteBase::make(pos, size, handle, Some(ball_body_handle));
    //     let sprite = make_shared(sprite);
    //     self.sprites.push(sprite.clone());
    //     sprite
    // }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn on_key_pressed(&mut self, key: Key, _action: Action) {
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
        self.sprites.push(make_shared(sprite))
    }

    pub fn add_collider(&mut self, sprite: SpriteBase) -> Shared<dyn Sprite> {
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let handle = self.collider_set.insert(collider);
        let collider = make_shared(Collider::make(sprite, handle));
        self.sprites.push(collider.clone());
        collider
    }
}

impl New for LevelBase {
    fn new() -> Self {
        LevelBase {
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
        }
    }
}
