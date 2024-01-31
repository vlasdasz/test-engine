use bytemuck::{Pod, Zeroable};

use crate::ColorBase;

pub type U8Color = ColorBase<u8>;
unsafe impl Zeroable for U8Color {}
unsafe impl Pod for U8Color {}
