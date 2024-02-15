use crate::{Alert, Spinner};

pub trait AlertErr<T> {
    fn alert_err(self) -> Option<T>;
}

impl<T, E: ToString> AlertErr<T> for Result<T, E> {
    fn alert_err(self) -> Option<T> {
        match self {
            Err(err) => {
                Spinner::instant_stop();
                Alert::show(err);
                None
            }
            Ok(val) => val.into(),
        }
    }
}

impl AlertErr<()> for () {
    fn alert_err(self) -> Option<()> {
        None
    }
}
