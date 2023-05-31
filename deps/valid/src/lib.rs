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
    fn rules() -> Vec<ValidEntry<Self>>;

    fn is_valid(&self) -> ValidResult<()> {
        for rule in Self::rules() {
            rule.check(self)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use reflected::Reflected;
    use reflected_proc::Reflected;

    use crate::{valid_reflected::ValidReflected, Valid, ValidEntry, ValidError, ValidRule};

    #[test]
    fn size() {
        #[derive(Default, Debug, Reflected)]
        pub struct User {
            name: String,
            age:  usize,
        }

        let user = User {
            name: "peter".into(),
            age:  15,
        };

        assert_eq!(user.size(&User::FIELDS.name), 5);
        assert_eq!(user.size(&User::FIELDS.age), 15);
    }

    #[test]
    fn no_rules() {
        #[derive(Default, Debug, Reflected)]
        pub struct User {}

        impl Valid for User {
            fn rules() -> Vec<ValidEntry<User>> {
                vec![]
            }
        }

        let user = User::default();
        assert_eq!(user.is_valid(), Ok(()));
    }

    #[test]
    fn min() {
        #[derive(Default, Debug, Reflected)]
        pub struct User {
            age: usize,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry<User>> {
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
        #[derive(Default, Debug, Reflected)]
        pub struct User {
            age: usize,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry<Self>> {
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

    #[test]
    fn range() {
        #[derive(Default, Debug, Reflected)]
        pub struct User {
            age: usize,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry<Self>> {
                vec![ValidEntry::new(&User::FIELDS.age, ValidRule::Range(14, 18))]
            }
        }

        let too_young1 = User { age: 5 };
        let too_young2 = User { age: 13 };
        let ok = User { age: 14 };
        let ok2 = User { age: 15 };
        let ok3 = User { age: 18 };
        let too_old1 = User { age: 19 };
        let too_old2 = User { age: 50 };

        assert_eq!(too_young1.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(too_young2.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(ok.is_valid(), Ok(()));
        assert_eq!(ok2.is_valid(), Ok(()));
        assert_eq!(ok3.is_valid(), Ok(()));
        assert_eq!(too_old1.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(too_old2.is_valid(), Err(ValidError::BadStuff));
    }

    #[test]
    fn equals() {
        #[derive(Default, Debug, Reflected)]
        pub struct User {
            pass:         String,
            confirm_pass: String,
            code:         u32,
            confirm_code: u32,
        }

        impl Valid for User {
            fn rules() -> Vec<ValidEntry<Self>> {
                vec![
                    ValidEntry::new(&User::FIELDS.pass, ValidRule::Equals(&User::FIELDS.confirm_pass)),
                    ValidEntry::new(&User::FIELDS.code, ValidRule::Equals(&User::FIELDS.confirm_code)),
                ]
            }
        }

        let mut bad_pass = User::random();
        bad_pass.confirm_pass = "aaaaaaaaaaaa".into();

        let mut bad_code = User::random();
        bad_code.confirm_code = 1111111111;

        let mut ok = User::random();
        ok.confirm_pass = ok.pass.clone();
        ok.confirm_code = ok.code;

        assert_eq!(bad_pass.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(bad_code.is_valid(), Err(ValidError::BadStuff));
        assert_eq!(ok.is_valid(), Ok(()));
    }
}
