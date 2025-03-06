use std::{
    future::Future,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use gm::ToF32;
use log::warn;
use refs::is_main_thread;
use tokio::{
    spawn,
    sync::oneshot::{Sender, channel},
};

type Callback = Box<dyn FnOnce() + Send>;
type Callbacks = Mutex<Vec<Callback>>;
type SignalledCallbacks = Mutex<Vec<(Sender<()>, Callback)>>;

static CALLBACKS: Callbacks = Callbacks::new(vec![]);
static SIGNALLED: SignalledCallbacks = SignalledCallbacks::new(vec![]);

pub async fn from_main<T, A>(action: A) -> T
where
    A: FnOnce() -> T + Send + 'static,
    T: Send + 'static, {
    assert!(
        !is_main_thread(),
        "This is already main thread. Just call it without `from_main`"
    );

    let result = Arc::<Mutex<Option<T>>>::default();

    let (sender, receiver) = channel::<()>();

    let capture = result.clone();
    SIGNALLED.lock().unwrap().push((
        sender,
        Box::new(move || {
            let mut res = capture.lock().unwrap();
            *res = action().into();
        }),
    ));

    receiver.await.expect("Failed to receive result in on_main");

    result.lock().unwrap().take().unwrap()
}

pub async fn wait_for_next_frame() {
    from_main(|| {}).await;
}

pub fn on_main(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        CALLBACKS.lock().unwrap().push(Box::new(action));
    }
}

pub fn on_main_sync(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        let (sender, mut receiver) = channel::<()>();
        SIGNALLED.lock().unwrap().push((sender, Box::new(action)));
        while receiver.try_recv().is_err() {}
    }
}

pub fn after(delay: impl ToF32, action: impl FnOnce() + Send + 'static) {
    spawn(async move {
        sleep(Duration::from_secs_f32(delay.to_f32()));
        CALLBACKS.lock().unwrap().push(Box::new(action));
    });
}

pub fn async_after(delay: impl ToF32, action: impl Future + Send + 'static) {
    spawn(async move {
        sleep(Duration::from_secs_f32(delay.to_f32()));
        action.await;
    });
}

pub fn invoke_dispatched() {
    let Ok(mut callback) = CALLBACKS.try_lock() else {
        warn!("Failed to lock CALLBACKS");
        return;
    };

    for action in callback.drain(..) {
        action();
    }
    drop(callback);

    let Ok(mut signalled) = SIGNALLED.try_lock() else {
        warn!("Failed to lock SIGNALLED");
        return;
    };

    for (signal, action) in signalled.drain(..) {
        action();
        signal.send(()).unwrap();
    }
}
