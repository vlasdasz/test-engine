use gm::flat::Points;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer,
};

use crate::WGPUApp;

#[derive(Debug)]
pub struct PathData {
    pub buffer: Buffer,
}

impl From<Points> for PathData {
    fn from(value: Points) -> Self {
        let buffer = WGPUApp::current()
            .state
            .drawer
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label:    "PathData Buffer".into(),
                contents: bytemuck::cast_slice(&value),
                usage:    wgpu::BufferUsages::VERTEX,
            });

        Self { buffer }
    }
}
