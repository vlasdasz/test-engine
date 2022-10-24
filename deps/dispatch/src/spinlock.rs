use std::sync::Mutex;

pub(crate) struct SpinLock {
    locked: Mutex<bool>,
}

impl SpinLock {
    pub(crate) fn locked(&self) -> bool {
        *self.locked.lock().unwrap()
    }

    pub(crate) fn lock(&self) {
        *self.locked.lock().unwrap() = true;
    }

    pub(crate) fn unlock(&self) {
        *self.locked.lock().unwrap() = false;
    }

    pub(crate) fn wait(&self) {
        while self.locked() {}
    }
}

impl const Default for SpinLock {
    fn default() -> Self {
        Self {
            locked: Mutex::new(false),
        }
    }
}
