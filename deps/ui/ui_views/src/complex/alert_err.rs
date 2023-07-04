use crate::{Alert, Spinner};

pub trait AlertErr {
    fn alert_err(self);
}

impl<T, E: ToString> AlertErr for Result<T, E> {
    fn alert_err(self) {
        if let Err(err) = self {
            Spinner::instant_stop();
            Alert::show(err);
        }
    }
}

impl AlertErr for () {
    fn alert_err(self) {}
}
