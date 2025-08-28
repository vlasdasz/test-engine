// use dispatch::Task;
use ui::{AlertErr, Spinner};

pub trait TaskSpinner<T> {
    fn spin(task: impl FnOnce() -> T + Send + 'static);
}

// impl<T: Send + AlertErr<()> + 'static> TaskSpinner<T> for Task<T> {
//     /// Runs a task and shows a spinner while it is running.
//     /// Errors will be shown as alert.
//     fn spin(task: impl FnOnce() -> T + Send + 'static) {
//         let spin = Spinner::lock();
//         Self::blocking(task).callback(|result| {
//             drop(spin);
//             result.alert_err();
//         });
//     }
// }
