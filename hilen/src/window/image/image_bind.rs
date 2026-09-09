use wgpu::BindGroup;
#[cfg(feature = "scene")]
use wgpu::Sampler;
#[cfg(any(feature = "scene", feature = "video"))]
use wgpu::TextureView;

/// The sampled side of an image: the bind group the rect pipelines set
/// and the view and sampler a pipeline with a wider layout, like the
/// mesh one with its two textures, binds itself.
#[derive(Debug)]
pub(crate) struct ImageBind {
    pub(crate) bind:    BindGroup,
    #[cfg(any(feature = "scene", feature = "video"))]
    pub(crate) view:    TextureView,
    #[cfg(feature = "scene")]
    pub(crate) sampler: Sampler,
}

#[cfg(wasm)]
unsafe impl Send for ImageBind {}
