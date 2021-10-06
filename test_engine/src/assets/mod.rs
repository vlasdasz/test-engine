mod buffers;
mod shaders;

pub use buffers::Buffers;
pub use shaders::Shaders;

#[derive(Default)]
pub struct Assets {
    pub buffers: Buffers,
    pub shaders: Shaders,
}
