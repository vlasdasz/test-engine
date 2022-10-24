use std::sync::Mutex;

use tokio::sync::oneshot::{channel, Receiver, Sender};

use crate::SpinLock;

static LOCK: SpinLock = Default::default();

static SIGNAL: Mutex<Option<Sender<()>>> = Mutex::new(None);

pub struct MainLock {}

impl MainLock {
    fn start_lock() -> Receiver<()> {
        let mut signal = SIGNAL.lock().unwrap();

        if signal.is_some() {
            panic!("AAAA!!");
        }

        let (sender, receiver) = channel();

        *signal = sender.into();

        receiver
    }

    pub async fn new() -> Self {
        Self::start_lock().await.unwrap();
        Self {}
    }
}

impl MainLock {
    pub fn wait() {
        let mut signal = SIGNAL.lock().unwrap();

        let Some(sender) = signal.take() else {
            return;
        };

        LOCK.lock();

        sender.send(()).unwrap();

        LOCK.wait();
    }
}

impl Drop for MainLock {
    fn drop(&mut self) {
        LOCK.unlock()
    }
}

pub async fn lock_main() -> MainLock {
    MainLock::new().await
}
