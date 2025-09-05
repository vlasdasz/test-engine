use std::sync::{
    Arc, Mutex,
    mpsc::{Sender, channel},
};

use anyhow::Result;
use gm::ToF32;
use log::warn;
use refs::is_main_thread;

type Callback = Box<dyn FnOnce() + Send>;
type Callbacks = Mutex<Vec<Callback>>;
type SignalledCallbacks = Mutex<Vec<(Sender<()>, Callback)>>;

static CALLBACKS: Callbacks = Callbacks::new(vec![]);
static SIGNALLED: SignalledCallbacks = SignalledCallbacks::new(vec![]);

pub fn from_main<T, A>(action: A) -> T
where
    A: FnOnce() -> T + Send + 'static,
    T: Send + 'static, {
    if is_main_thread() {
        return action();
    }

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

    receiver.recv().expect("Failed to receive result in on_main");

    result.lock().unwrap().take().unwrap()
}

pub fn wait_for_next_frame() {
    assert!(
        !is_main_thread(),
        "Waiting for next frame on main thread does nothing"
    );
    from_main(|| {});
}

pub fn on_main(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        CALLBACKS.lock().unwrap().push(Box::new(action));
    }
}

pub fn ok_main(action: impl FnOnce() + Send + 'static) -> Result<()> {
    on_main(action);
    Ok(())
}

pub fn on_main_sync(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        let (sender, receiver) = channel::<()>();
        SIGNALLED.lock().unwrap().push((sender, Box::new(action)));
        while receiver.try_recv().is_err() {}
    }
}

pub fn after(delay: impl ToF32, action: impl FnOnce() + Send + 'static) {
    crate::spawn(async move {
        crate::sleep(delay).await;
        CALLBACKS.lock().unwrap().push(Box::new(action));
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
