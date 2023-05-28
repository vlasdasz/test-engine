use reflected::{Field, Reflected};

use crate::{valid_reflected::ValidReflected, ValidError, ValidResult};

pub enum ValidRule {
    Min(usize),
    Max(usize),
    Range(usize, usize),
    Equals(&'static Field<'static>),
}

impl ValidRule {
    pub(crate) fn check<T: Reflected>(&self, obj: &T, field: &'static Field) -> ValidResult<()> {
        match self {
            Self::Min(min) => Self::check_min(*min, obj, field),
            Self::Max(max) => Self::check_max(*max, obj, field),
            Self::Range(min, max) => Self::check_range(*min, *max, obj, field),
            Self::Equals(other_field) => Self::check_equals(other_field, obj, field),
        }
    }
}

impl ValidRule {
    fn check_min<T: Reflected>(min: usize, obj: &T, field: &'static Field) -> ValidResult<()> {
        if obj.size(field) < min {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_max<T: Reflected>(max: usize, obj: &T, field: &'static Field) -> ValidResult<()> {
        if obj.size(field) > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_range<T: Reflected>(min: usize, max: usize, obj: &T, field: &'static Field) -> ValidResult<()> {
        let size = obj.size(field);
        if size < min || size > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_equals<T: Reflected>(
        other_field: &'static Field,
        obj: &T,
        field: &'static Field,
    ) -> ValidResult<()> {
        if obj.get_value(other_field) == obj.get_value(field) {
            Ok(())
        } else {
            Err(ValidError::BadStuff)
        }
    }
}
