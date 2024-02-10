use reflected::Field;

pub trait _ValidField<T> {}

impl<T> _ValidField<T> for Field<'_, T> {}
