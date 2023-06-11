use crate::Alert;

pub trait AlertErr<T> {
    fn alert_err(self) -> Result<T, String>;
}

impl<Err: ToString, T> AlertErr<T> for Result<T, Err> {
    fn alert_err(self) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => {
                Alert::show(err);
                Err("alert_handled".to_string())
            }
        }
    }
}
