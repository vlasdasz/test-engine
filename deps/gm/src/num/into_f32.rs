use std::fmt::{Debug, Display};

use crate::LossyConvert;

#[const_trait]
pub trait ToF32: Copy + Sized + Sync + Send + Debug + Display + 'static {
    fn to_f32(self) -> f32;
}

impl const ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}

impl const ToF32 for i64 {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}

impl const ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}

impl const ToF32 for u64 {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}

impl const ToF32 for usize {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}

impl const ToF32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl const ToF32 for f64 {
    fn to_f32(self) -> f32 {
        self.lossy_convert()
    }
}
