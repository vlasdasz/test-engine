use rapier2d::prelude::{ColliderSet, RigidBodySet};

pub struct Sets {
    pub rigid_body: RigidBodySet,
    pub collider:   ColliderSet,
}

impl Default for Sets {
    fn default() -> Self {
        Self {
            rigid_body: RigidBodySet::new(),
            collider:   ColliderSet::new(),
        }
    }
}
