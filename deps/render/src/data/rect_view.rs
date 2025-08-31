use bytemuck::{Pod, Zeroable};
use gm::flat::Size;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone, Zeroable, Pod)]
pub struct RectView {
    pub resolution: Size,
    pub _padding:   u64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Web requirements
        assert_eq!(size_of::<RectView>() % 16, 0);
    }
}
