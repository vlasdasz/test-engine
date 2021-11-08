use rapier2d::{dynamics::RigidBodyHandle, parry::partitioning::IndexedData};

pub struct RigidHandle {
    pub handle: RigidBodyHandle,
}

impl Default for RigidHandle {
    fn default() -> Self {
        Self {
            handle: RigidBodyHandle(rapier2d::data::arena::Index::default()),
        }
    }
}
