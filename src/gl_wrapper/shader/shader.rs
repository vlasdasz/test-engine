use std::path::PathBuf;

use super::shader_compiler::compile;

pub struct Shader {
    pub program: u32
}

impl Shader {
    pub fn new(path: PathBuf) -> Shader {
        Shader { program: compile(path) }
    }
}