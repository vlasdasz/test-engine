#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

pub trait LossyConvert<To> {
    fn lossy_convert(self) -> To;
}

impl LossyConvert<u32> for f32 {
    fn lossy_convert(self) -> u32 {
        assert!(self >= 0.0, "Lossy convert sign loss");
        assert!(self <= u32::MAX as f32, "Lossy convert overflow");
        self as u32
    }
}

impl LossyConvert<usize> for f32 {
    fn lossy_convert(self) -> usize {
        assert!(self >= 0.0, "Lossy convert sign loss");
        assert!(self <= usize::MAX as f32, "Lossy convert overflow");
        self as usize
    }
}

impl LossyConvert<isize> for f32 {
    fn lossy_convert(self) -> isize {
        assert!(self <= isize::MAX as f32, "Lossy convert overflow");
        assert!(self > isize::MIN as f32, "Lossy convert underflow");
        self as isize
    }
}

impl LossyConvert<u8> for f32 {
    fn lossy_convert(self) -> u8 {
        assert!(self >= 0.0, "Lossy convert sign loss");
        assert!(self <= f32::from(u8::MAX), "Lossy convert overflow");
        self as u8
    }
}
