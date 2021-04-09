
#[cfg(target_os="ios")]
use gles31_sys::*;

use crate::gm::*;

pub struct Shader {
    program: u32,
    color:           i32,
    mvp_matrix:      i32,
    model_matrix:    i32,
    light_position:  i32,
    size:            i32,
    selected:        i32,
    resolution:      i32,
    position:        i32,
    rotation:        i32,
    camera_position: i32,
    flip:            i32
}

fn get_uniform(program: u32, lit: &str) -> i32 {
    use std::ffi::CString;
    let c_str = CString::new(lit).unwrap();
    GL!(GetUniformLocation, program, c_str.as_ptr())
}

impl Shader {

    pub fn new(program: u32) -> Shader {
        Shader {
            program,
            color:           get_uniform(program, "color"),
            mvp_matrix:      get_uniform(program, "mvp_matrix"),
            model_matrix:    get_uniform(program, "model_matrix"),
            light_position:  get_uniform(program, "light_position"),
            size:            get_uniform(program, "size"),
            selected:        get_uniform(program, "selected"),
            resolution:      get_uniform(program, "resolution"),
            position:        get_uniform(program, "position"),
            rotation:        get_uniform(program, "rotation"),
            camera_position: get_uniform(program, "camera_position"),
            flip:            get_uniform(program, "flip")
        }
    }

    pub fn enable(&self) {
        GL!(UseProgram, self.program)
    }

    pub fn set_color(&self, color: &Color) {
        GL!(Uniform4fv, self.color, 1, &color.r)
    }

    pub fn set_size(&self, size: &Size) {
        GL!(Uniform2fv, self.size, 1, &size.width)
    }

    pub fn set_selected(&self, selected: bool) {
        GL!(Uniform1i, self.selected, selected.into())
    }

    pub fn set_resolution(&self, resolution: &Size) {
        GL!(Uniform2fv, self.resolution, 1, &resolution.width)
    }

    pub fn set_position(&self, point: &Point) {
        GL!(Uniform2fv, self.position, 1, &point.x)
    }

    pub fn set_rotation(&self, angle: f32) {
        GL!(Uniform1f, self.rotation, angle)
    }

    pub fn set_camera_position(&self, pos: &Point) {
        GL!(Uniform2fv, self.camera_position, 1, &pos.x)
    }

    pub fn set_flip(&self, flip: bool) {
        GL!(Uniform1i, self.flip, flip.into())
    }

}