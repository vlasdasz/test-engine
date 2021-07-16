use crate::gm::{Color, Point, Size};
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::prelude::ColliderHandle;

pub struct Sprite {
    pub position: Point,
    pub size: Size,
    pub rotation: f32,
    pub color: Color,
    pub collider_handle: ColliderHandle,
    pub rigid_body_handle: Option<RigidBodyHandle>,
}

impl Sprite {
    pub fn new(
        position: Point,
        size: Size,
        collider_handle: ColliderHandle,
        rigid_body_handle: Option<RigidBodyHandle>,
    ) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            color: Color::random().clone(),
            collider_handle,
            rigid_body_handle,
        }
    }
}
