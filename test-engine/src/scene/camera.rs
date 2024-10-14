#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub(crate) struct Camera {
    pub(crate) eye:    cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up:     cgmath::Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy:   f32,
    pub(crate) znear:  f32,
    pub(crate) zfar:   f32,
}

impl Camera {
    pub(crate) fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
