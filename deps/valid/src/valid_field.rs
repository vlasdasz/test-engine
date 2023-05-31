use reflected::Field;

pub(crate) trait ValidField<T> {}

impl<T> ValidField<T> for Field<'_, T> {}
