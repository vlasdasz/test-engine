use reflected::{Field, Reflected};

use crate::{valid_reflected::ValidReflected, ValidError, ValidResult};

pub enum ValidRule<T: 'static> {
    Min(usize),
    Max(usize),
    Range(usize, usize),
    Equals(&'static Field<'static, T>),
}

impl<T: Reflected> ValidRule<T> {
    pub(crate) fn check(&self, obj: &T, field: &'static Field<T>) -> ValidResult<()> {
        match self {
            Self::Min(min) => Self::check_min(*min, obj, field),
            Self::Max(max) => Self::check_max(*max, obj, field),
            Self::Range(min, max) => Self::check_range(*min, *max, obj, field),
            Self::Equals(other_field) => Self::check_equals(other_field, obj, field),
        }
    }
}

impl<T: Reflected> ValidRule<T> {
    fn check_min(min: usize, obj: &T, field: &'static Field<T>) -> ValidResult<()> {
        if obj.size(field) < min {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_max(max: usize, obj: &T, field: &'static Field<T>) -> ValidResult<()> {
        if obj.size(field) > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_range(min: usize, max: usize, obj: &T, field: &'static Field<T>) -> ValidResult<()> {
        let size = obj.size(field);
        if size < min || size > max {
            Err(ValidError::BadStuff)
        } else {
            Ok(())
        }
    }

    fn check_equals(other_field: &'static Field<T>, obj: &T, field: &'static Field<T>) -> ValidResult<()> {
        if obj.get_value(other_field) == obj.get_value(field) {
            Ok(())
        } else {
            Err(ValidError::BadStuff)
        }
    }
}
