use reflected::{Field, Reflected};

pub(crate) trait ValidReflected: Sized {
    fn size(&self, field: &'static Field<Self>) -> usize;
}

impl<T: ?Sized + Reflected> ValidReflected for T {
    fn size(&self, field: &'static Field<Self>) -> usize {
        let val = self.get_value(field);
        if field.is_number() {
            val.parse::<usize>().unwrap()
        } else {
            val.len()
        }
    }
}
