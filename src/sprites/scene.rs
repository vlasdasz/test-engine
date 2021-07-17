use crate::gm;
use crate::sprites::Sprite;
use rapier2d::na::Vector2;
use tools::refs::{make_shared, Shared};
use tools::New;

use rapier2d::prelude::{
    BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager,
    JointSet, NarrowPhase, PhysicsPipeline, RigidBodyBuilder, RigidBodySet,
};

pub struct Scene {
    pub sprites: Vec<Shared<Sprite>>,
    pub walls: Vec<Shared<Sprite>>,

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

impl Scene {
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

        for sprite in &self.sprites {
            let mut sprite = sprite.borrow_mut();

            let body = &self.rigid_body_set[sprite.rigid_body_handle.unwrap()];

            sprite.position.x = body.translation().x;
            sprite.position.y = body.translation().y;
            sprite.rotation = body.rotation().angle();
        }
    }

    pub fn add_collider(&mut self, pos: gm::Point, size: gm::Size) -> Shared<Sprite> {
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        let handle = self.collider_set.insert(collider);
        let sprite = Sprite::new(pos, size, handle, None);
        let sprite = make_shared(sprite);
        self.walls.push(sprite.clone());
        sprite
    }

    pub fn add_ball(&mut self, pos: gm::Point, size: f32) -> Shared<Sprite> {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(pos.x, pos.y))
            .build();
        let collider = ColliderBuilder::ball(size).restitution(0.7).build();
        let ball_body_handle = self.rigid_body_set.insert(rigid_body);
        let handle = self.collider_set.insert_with_parent(
            collider,
            ball_body_handle,
            &mut self.rigid_body_set,
        );
        let sprite = Sprite::new(
            pos,
            gm::Size::make(size * 2.0, size * 2.0),
            handle,
            Some(ball_body_handle),
        );
        let sprite = make_shared(sprite);
        self.sprites.push(sprite.clone());
        sprite
    }
}

impl New for Scene {
    fn new() -> Self {
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        let gravity = Vector2::new(0.0, -9.81);
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let joint_set = JointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();
        Scene {
            sprites: vec![],
            walls: vec![],
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
        }
    }
}
