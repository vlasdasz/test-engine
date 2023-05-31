use reflected::{Field, Reflected};

use crate::{ValidResult, ValidRule};

pub struct ValidEntry<T: 'static> {
    field: &'static Field<'static, T>,
    rule:  ValidRule<T>,
}

impl<T: Reflected> ValidEntry<T> {
    pub fn new(field: &'static Field<T>, rule: ValidRule<T>) -> Self {
        Self { field, rule }
    }

    pub fn check(&self, obj: &T) -> ValidResult<()> {
        self.rule.check(obj, self.field)
    }
}
