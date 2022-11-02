pub mod error;
pub mod valid_entry;
mod valid_field;
mod valid_reflected;
pub mod valid_rule;

pub use error::*;
use reflected::Reflected;
pub use valid_entry::*;
pub use valid_rule::*;

pub trait Valid: Reflected {
    fn rules() -> Vec<ValidEntry>;

    fn is_valid(&self) -> ValidResult<()> {
        for rule in Self::rules() {
            rule.check(self)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use reflected_proc::reflected;

    use crate::{valid_reflected::ValidReflected, Valid, ValidEntry, ValidError, ValidRule};

    #[test]
    fn size() {
        #[reflected]
        #[derive(Default)]
        struct User {
            name: String,
            age:  usize,
        }

        let user = User {
            name: "peter".into(),
            age:  15,
        };

        assert_eq!(user.size(&User::FIELDS.age), 15);
        assert_eq!(user.size(&User::FIELDS.name), 5);
    }

    #[test]
    fn no_rules() {
        #[reflected]
        #[derive(Default)]
        struct User {}

        impl Valid for User {
            fn rules() -> Vec<ValidEntry> {
                vec![]
            }
        }

        let user = User::default();
        assert_eq!(user.is_valid(), Ok(()));
    }

    #[test]
    fn min() {
        #[reflected]
        #[derive(Default)]
        struct User {
            age: usize,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry> {
                vec![ValidEntry::new(&User::FIELDS.age, ValidRule::Min(14))]
            }
        }

        let too_young1 = User { age: 5 };
        let too_young2 = User { age: 10 };
        let ok = User { age: 14 };
        let ok2 = User { age: 50 };

        assert_eq!(too_young1.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(too_young2.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(ok.is_valid(), Ok(()));
        assert_eq!(ok2.is_valid(), Ok(()));
    }

    #[test]
    fn max() {
        #[reflected]
        #[derive(Default)]
        struct User {
            age: usize,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry> {
                vec![ValidEntry::new(&User::FIELDS.age, ValidRule::Max(14))]
            }
        }

        let too_old1 = User { age: 20 };
        let too_old2 = User { age: 15 };
        let ok = User { age: 14 };
        let ok2 = User { age: 10 };

        assert_eq!(too_old1.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(too_old2.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(ok.is_valid(), Ok(()));
        assert_eq!(ok2.is_valid(), Ok(()));
    }
}
