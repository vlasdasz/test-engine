use reflected::{Field, Reflected};

use crate::{valid_reflected::ValidReflected, ValidError, ValidResult};

pub enum ValidRule {
    Min(usize),
    Max(usize),
    Range(usize, usize),
    Equals(&'static Field),
}

impl ValidRule {
    pub(crate) fn check<T: Reflected>(&self, obj: &T, field: &Field) -> ValidResult<()> {
        match self {
            Self::Min(min) => self.check_min(*min, obj, field),
            Self::Max(max) => self.check_max(*max, obj, field),
            Self::Range(min, max) => self.check_range(*min, *max, obj, field),
            Self::Equals(other_field) => self.check_equals(other_field, obj, field),
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

    fn check_range<T: Reflected>(&self, min: usize, max: usize, obj: &T, field: &Field) -> ValidResult<()> {
        let size = obj.size(field);
        if size < min || size > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_equals<T: Reflected>(&self, other_field: &Field, obj: &T, field: &Field) -> ValidResult<()> {
        if obj.get_value(other_field) != obj.get_value(field) {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }
}
