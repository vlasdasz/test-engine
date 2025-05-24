use log::error;
use tokio::{spawn, task::spawn_blocking};

use crate::on_main;

pub struct Task<T> {
    task: Box<dyn FnOnce() -> T + Send>,
}

impl<T: Send + 'static> Task<T> {
    /// Creates a blocking task to run in background.
    pub fn blocking(task: impl FnOnce() -> T + Send + 'static) -> Self {
        Self { task: Box::new(task) }
    }

    /// Runs the task and calls callback when it is finished.
    /// Callback is executed on main thread and is safe to access UI elements
    /// from.
    pub fn callback(self, callback: impl FnOnce(T) + Send + 'static) {
        spawn(async {
            match spawn_blocking(self.task).await {
                Ok(result) => {
                    on_main(|| {
                        callback(result);
                    });
                }
                Err(error) => {
                    error!("Failed to finish blocking task. Error: {error}");
                }
            }
        });
    }
}
