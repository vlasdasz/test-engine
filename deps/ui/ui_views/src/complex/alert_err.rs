use crate::{Alert, Spinner};

pub trait AlertErr {
    fn alert_err(self);
}

impl<E: ToString> AlertErr for Result<(), E> {
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
