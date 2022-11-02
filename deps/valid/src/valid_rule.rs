use std::ops::Range;

use reflected::{Field, Reflected};

use crate::{valid_reflected::ValidReflected, ValidError, ValidResult};

pub enum ValidRule {
    Min(usize),
    Max(usize),
    Size(Range<usize>),
}

impl ValidRule {
    pub(crate) fn check<T: Reflected>(&self, obj: &T, field: &Field) -> ValidResult<()> {
        match self {
            Self::Min(min) => self.check_min(*min, obj, field),
            Self::Max(max) => self.check_max(*max, obj, field),
            Self::Size(range) => self.check_size(range, obj, field),
        }
    }
}

impl ValidRule {
    fn check_min<T: Reflected>(&self, min: usize, obj: &T, field: &Field) -> ValidResult<()> {
        if obj.size(field) < min {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_max<T: Reflected>(&self, max: usize, obj: &T, field: &Field) -> ValidResult<()> {
        if obj.size(field) > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_size<T: Reflected>(&self, range: &Range<usize>, obj: &T, field: &Field) -> ValidResult<()> {
        if range.contains(&obj.size(field)) {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }
}
