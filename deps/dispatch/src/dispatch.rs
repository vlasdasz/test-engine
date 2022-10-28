use std::sync::Mutex;

use rtools::{sleep, IntoF32};
use tokio::spawn;

type Storage = Mutex<Vec<Box<dyn FnOnce() + Send>>>;

static STORAGE: Storage = Storage::new(Default::default());

pub struct Dispatch;

impl Dispatch {
    pub fn main(action: impl FnOnce() + Send + 'static) {
        STORAGE.lock().unwrap().push(Box::new(action));
    }

    pub fn after(delay: impl IntoF32, action: impl FnOnce() + Send + 'static) {
        spawn(async move {
            sleep(delay);
            STORAGE.lock().unwrap().push(Box::new(action));
        });
    }

    pub fn call() {
        let mut data = STORAGE.lock().unwrap();
        while let Some(action) = data.pop() {
            action()
        }
    }
}
