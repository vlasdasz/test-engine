use reflected::{Field, Reflected};

pub(crate) trait ValidReflected {
    fn size(&self, field: &'static Field) -> usize;
}

impl<T: ?Sized + Reflected> ValidReflected for T {
    fn size(&self, field: &'static Field) -> usize {
        let val = self.get_value(field);
        if field.is_number() {
            val.parse::<usize>().unwrap()
        } else {
            val.len()
        }
    }
}
