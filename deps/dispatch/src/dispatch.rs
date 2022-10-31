use std::sync::{Arc, Mutex};

use rtools::{sleep, IntoF32};
use tokio::{
    spawn,
    sync::oneshot::{channel, Sender},
};

type Callbacks = Mutex<Vec<Box<dyn FnOnce() + Send>>>;
type SignalledCallbacks = Mutex<Vec<(Sender<()>, Box<dyn FnOnce() + Send>)>>;

static CALLBACKS: Callbacks = Callbacks::new(Default::default());
static SIGNALLED: SignalledCallbacks = SignalledCallbacks::new(Default::default());

pub async fn from_main<T, A>(action: A) -> T
where
    A: FnOnce() -> T + Send + 'static,
    T: Send + Sync + Default + 'static,
{
    let res = Arc::<Mutex<Option<T>>>::default();

    let (sender, receiver) = channel::<()>();

    let capture = res.clone();
    SIGNALLED.lock().unwrap().push((
        sender,
        Box::new(move || {
            let mut res = capture.lock().unwrap();
            *res = action().into();
        }),
    ));

    receiver.await.unwrap();

    let res = res.lock().unwrap().take().unwrap();
    res
}

pub fn on_main(action: impl FnOnce() + Send + 'static) {
    CALLBACKS.lock().unwrap().push(Box::new(action));
}

pub fn after(delay: impl IntoF32, action: impl FnOnce() + Send + 'static) {
    spawn(async move {
        sleep(delay);
        CALLBACKS.lock().unwrap().push(Box::new(action));
    });
}

pub fn invoke_dispatched() {
    let mut callback = CALLBACKS.lock().unwrap();
    while let Some(action) = callback.pop() {
        action()
    }
    drop(callback);
    let mut signalled = SIGNALLED.lock().unwrap();
    while let Some(action) = signalled.pop() {
        (action.1)();
        action.0.send(()).unwrap();
    }
}
