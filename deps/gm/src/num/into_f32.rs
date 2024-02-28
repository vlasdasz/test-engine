use std::fmt::{Debug, Display};

use crate::LossyConvert;

#[const_trait]
pub trait IntoF32: Copy + Sized + Sync + Send + Debug + Display + 'static {
    fn into_f32(self) -> f32;
}

impl const IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for usize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl const IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self.lossy_convert()
    }
}
