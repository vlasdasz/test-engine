use crate::gm::{Color, Point, Size};
use rapier2d::dynamics::{RigidBodyHandle, RigidBody};
use rapier2d::prelude::ColliderHandle;
use tools::{New, new};
use std::ptr::{null_mut};

pub struct Sprite {
    pub position: Point,
    pub size: Size,
    pub rotation: f32,
    pub color: Color,
    pub collider_handle: ColliderHandle,
    pub rigid_body_handle: Option<RigidBodyHandle>,
    rigid_body: *mut RigidBody,
}

impl Sprite {
    pub fn make(
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
            rigid_body: null_mut()
        }
    }
}

impl New for Sprite {
    fn new() -> Self {
        Sprite {
            position: new(),
            size: new(),
            rotation: 0.0,
            color: new(),
            collider_handle: ColliderHandle::invalid(),
            rigid_body_handle: None,
            rigid_body: null_mut()
        }
    }
}
