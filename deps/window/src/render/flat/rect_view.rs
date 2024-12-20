use bytemuck::{Pod, Zeroable};
use gm::flat::Size;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone, Zeroable, Pod)]
pub(super) struct RectView {
    pub resolution: Size,
}
