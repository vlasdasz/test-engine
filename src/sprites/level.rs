use crate::gm;
use crate::sprites::Sprite;
use rapier2d::na::Vector2;
use tools::refs::{make_shared, new_shared, Shared};
use tools::New;

use crate::image::Image;
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager,
    JointSet, NarrowPhase, PhysicsPipeline, RigidBody, RigidBodyBuilder, RigidBodySet,
};

pub trait Control {
    fn jump(&mut self);
    fn go_left(&mut self);
    fn go_right(&mut self);
}

pub struct Level {
    pub sprites: Vec<Shared<Sprite>>,
    pub walls: Vec<Shared<Sprite>>,

    pub player: Shared<Sprite>,

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

impl Level {
    pub fn setup(&mut self) {
        let player = self.add_rect(
            gm::Point::make(0, 10),
            gm::Size::make(17.0 / 6.0, 28.0 / 6.0),
        );
        self.player = player;
        self.player.borrow_mut().image =
            Some(Image::load(&crate::te::paths::images().join("frisk.png")));
        let body = self.player_body();
        body.lock_rotations(true, true);
        dbg!(body.mass());
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

        for sprite in &self.sprites {
            let mut sprite = sprite.borrow_mut();

            let body = &self.rigid_body_set[sprite.rigid_body_handle.unwrap()];

            sprite.position.x = body.translation().x;
            sprite.position.y = body.translation().y;
            sprite.rotation = body.rotation().angle();
        }
    }

    pub fn add_collider(&mut self, pos: gm::Point, size: gm::Size) -> Shared<Sprite> {
        let collider = ColliderBuilder::cuboid(size.width, size.height)
            .translation(Vector2::new(pos.x, pos.y))
            .build();
        let handle = self.collider_set.insert(collider);
        let sprite = Sprite::make(pos, size, handle, None);
        let sprite = make_shared(sprite);
        self.walls.push(sprite.clone());
        sprite
    }

    pub fn add_rect(&mut self, pos: gm::Point, size: gm::Size) -> Shared<Sprite> {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(pos.x, pos.y))
            .build();
        let collider = ColliderBuilder::cuboid(size.width, size.height)
            .restitution(0.7)
            .build();
        let ball_body_handle = self.rigid_body_set.insert(rigid_body);
        let handle = self.collider_set.insert_with_parent(
            collider,
            ball_body_handle,
            &mut self.rigid_body_set,
        );
        let sprite = Sprite::make(pos, size, handle, Some(ball_body_handle));
        let sprite = make_shared(sprite);
        self.sprites.push(sprite.clone());
        sprite
    }

    fn player_body(&mut self) -> &mut RigidBody {
        &mut self.rigid_body_set[self.player.borrow().rigid_body_handle.unwrap()]
    }
}

impl New for Level {
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
        Level {
            sprites: vec![],
            walls: vec![],
            player: new_shared(),
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

impl Control for Level {
    fn jump(&mut self) {
        self.player_body().set_linvel([0.0, 50.0].into(), true);
    }

    fn go_left(&mut self) {
        self.player_body().set_linvel([-50.0, 0.0].into(), true);
    }

    fn go_right(&mut self) {
        self.player_body().set_linvel([50.0, 0.0].into(), true);
    }
}
