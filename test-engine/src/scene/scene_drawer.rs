use ui::UIManager;

use crate::scene::camera::Camera;

pub(crate) struct SceneDrawer {
    pub(crate) camera: Camera,
}

impl Default for SceneDrawer {
    fn default() -> Self {
        let resolution = UIManager::resolution();

        Self {
            camera: Camera {
                // position the camera 1 unit up and 2 units back
                // +z is out of the screen
                eye:    (0.0, 1.0, 2.0).into(),
                // have it look at the origin
                target: (0.0, 0.0, 0.0).into(),
                // which way is "up"
                up:     cgmath::Vector3::unit_y(),
                aspect: resolution.width / resolution.height,
                fovy:   45.0,
                znear:  0.1,
                zfar:   100.0,
            },
        }
    }
}
