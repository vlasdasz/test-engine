pub trait FloatIterExt {
    fn float_min(&mut self) -> f32;
    fn float_max(&mut self) -> f32;
}

impl<T> FloatIterExt for T
where T: Iterator<Item = f32>
{
    fn float_max(&mut self) -> f32 {
        self.fold(f32::NAN, f32::max)
    }

    fn float_min(&mut self) -> f32 {
        self.fold(f32::NAN, f32::min)
    }
}
