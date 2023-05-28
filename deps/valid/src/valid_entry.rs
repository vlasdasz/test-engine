use reflected::{Field, Reflected};

use crate::{ValidResult, ValidRule};

pub struct ValidEntry {
    field: &'static Field<'static>,
    rule:  ValidRule,
}

impl ValidEntry {
    pub fn new(field: &'static Field, rule: ValidRule) -> Self {
        Self { field, rule }
    }

    pub fn check<T: Reflected>(&self, obj: &T) -> ValidResult<()> {
        self.rule.check(obj, self.field)
    }
}
