use crate::gm::{Color, Point, Size};
use crate::image::Image;
use rapier2d::dynamics::{RigidBody, RigidBodyHandle};
use rapier2d::prelude::ColliderHandle;
use std::ptr::null_mut;
use tools::{new, New};

pub struct Sprite {
    pub position: Point,
    pub size: Size,
    pub rotation: f32,
    pub color: Color,
    pub collider_handle: ColliderHandle,
    pub rigid_body_handle: Option<RigidBodyHandle>,
    pub image: Option<Image>,
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
            image: None,
            rigid_body: null_mut(),
        }
    }
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image)
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
            image: None,
            rigid_body: null_mut(),
        }
    }
}
