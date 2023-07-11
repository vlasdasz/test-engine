#[cfg(mobile)]
use gles31_sys::*;
use gm::{
    flat::{Point, Size},
    Color,
};

#[derive(Debug)]
pub struct Shader {
    name: String,

    program:    u32,
    color:      i32,
    size:       i32,
    selected:   i32,
    resolution: i32,
    position:   i32,
    rotation:   i32,
    flipped:    i32,
    flipped_y:  i32,
    scale:      i32,
    z_position: i32,

    camera_rotation: i32,
    camera_position: i32,
}

fn get_uniform(program: u32, lit: &str) -> i32 {
    use std::ffi::CString;
    let c_str = CString::new(lit).unwrap();
    GL!(GetUniformLocation, program, c_str.as_ptr())
}

impl Shader {
    pub fn new(program: u32, name: impl ToString) -> Shader {
        Shader {
            name: name.to_string(),
            program,
            color: get_uniform(program, "color"),
            size: get_uniform(program, "size"),
            selected: get_uniform(program, "selected"),
            resolution: get_uniform(program, "resolution"),
            position: get_uniform(program, "position"),
            rotation: get_uniform(program, "rotation"),
            flipped: get_uniform(program, "flipped"),
            flipped_y: get_uniform(program, "flipped_y"),
            scale: get_uniform(program, "scale"),
            camera_rotation: get_uniform(program, "camera_rotation"),
            camera_position: get_uniform(program, "camera_position"),
            z_position: get_uniform(program, "z_position"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enable(&self) -> &Self {
        GL!(UseProgram, self.program);
        self
    }

    pub fn set_color(&self, color: &Color) -> &Self {
        debug_assert!(self.color >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform4fv, self.color, 1, &color.r);
        self
    }

    pub fn set_size(&self, size: Size) -> &Self {
        debug_assert!(self.size >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform2fv, self.size, 1, &size.width);
        self
    }

    pub fn set_selected(&self, selected: bool) -> &Self {
        debug_assert!(self.selected >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1i, self.selected, selected.into());
        self
    }

    pub fn set_resolution(&self, resolution: Size) -> &Self {
        debug_assert!(self.resolution >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform2fv, self.resolution, 1, &resolution.width);
        self
    }

    pub fn set_position(&self, point: Point) -> &Self {
        debug_assert!(self.position >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform2fv, self.position, 1, &point.x);
        self
    }

    pub fn set_rotation(&self, angle: f32) -> &Self {
        debug_assert!(self.rotation >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1f, self.rotation, angle);
        self
    }

    pub fn set_camera_rotation(&self, angle: f32) {
        debug_assert!(self.camera_position >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1f, self.camera_rotation, angle)
    }

    pub fn set_camera_position(&self, pos: Point) {
        debug_assert!(self.camera_position >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform2fv, self.camera_position, 1, &pos.x)
    }

    pub fn set_flipped(&self, flipper: bool) -> &Self {
        debug_assert!(self.flipped >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1i, self.flipped, flipper.into());
        self
    }

    pub fn set_flipped_y(&self, flipper: bool) -> &Self {
        debug_assert!(self.flipped_y >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1i, self.flipped_y, flipper.into());
        self
    }

    pub fn set_scale(&self, scale: f32) -> &Self {
        debug_assert!(self.scale >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1f, self.scale, scale);
        self
    }

    pub fn set_z_position(&self, z_position: f32) -> &Self {
        debug_assert!(self.z_position >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1f, self.z_position, z_position);
        self
    }

    pub fn set_priority(&self, priority: usize) -> &Self {
        debug_assert!(self.z_position >= 0, "Invalid shader uniform: {}", self.name);
        GL!(Uniform1f, self.z_position, 0.5 - (priority as f32) / 1000.0);
        self
    }
}
