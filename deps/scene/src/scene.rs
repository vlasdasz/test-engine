use gm::flat::Size;

use crate::{camera::Camera, object::Object};

pub struct Scene {
    pub camera:  Camera,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn set_aspect_ratio(&mut self, resolution: Size) {
        self.camera.aspect = resolution.width / resolution.height;
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera:  Camera {
                // position the camera 1 unit up and 2 units back
                // +z is out of the screen
                eye:    (0.0, 1.0, 2.0).into(),
                // have it look at the origin
                target: (0.0, 0.0, 0.0).into(),
                // which way is "up"
                up:     cgmath::Vector3::unit_y(),
                aspect: 1.0,
                fovy:   45.0,
                znear:  0.1,
                zfar:   100.0,
            },
            objects: Vec::new(),
        }
    }
}
