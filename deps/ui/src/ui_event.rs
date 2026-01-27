use parking_lot::{Mutex, MutexGuard};
use refs::{RawPointer, Weak};

struct Subscriber<T: Send> {
    subscriber: Weak,
    action:     Box<dyn FnMut(T) + Send>,
}

pub struct UIEvent<T: Send = ()> {
    subscribers: Mutex<Vec<Subscriber<T>>>,
    /// This allows unsibscribing from the event during its execution
    unsubscribe: Mutex<Vec<RawPointer>>,
}

impl<T: Send> Default for UIEvent<T> {
    fn default() -> Self {
        Self {
            subscribers: Mutex::default(),
            unsubscribe: Mutex::default(),
        }
    }
}

impl<T: Send> UIEvent<T> {
    pub const fn const_new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
            unsubscribe: Mutex::new(Vec::new()),
        }
    }

    fn clear_subscribers(&self, subs: &mut MutexGuard<Vec<Subscriber<T>>>) -> &Self {
        let mut unsubscribe = self.unsubscribe.lock();
        subs.retain(|a| !unsubscribe.contains(&a.subscriber.raw()) && a.subscriber.is_ok());
        unsubscribe.clear();
        self
    }

    pub fn sub<U: ?Sized>(&self, subscriber: Weak<U>, mut action: impl FnMut() + Send + 'static) {
        self.val(subscriber, move |_| action());
    }

    pub fn val<U: ?Sized>(&self, subscriber: Weak<U>, action: impl FnMut(T) + Send + 'static) {
        let mut subs = self.subscribers.lock();
        self.clear_subscribers(&mut subs);

        // This view is already subscribed
        if subs.iter().any(|s| s.subscriber.raw() == subscriber.raw()) {
            return;
        }

        subs.push(Subscriber {
            subscriber: subscriber.erase(),
            action:     Box::new(action),
        });
    }

    pub fn unsibscribe<U: ?Sized>(&self, view: Weak<U>) {
        self.unsubscribe.lock().push(view.raw());
    }

    pub fn trigger(&self, val: T)
    where T: Clone {
        let mut subs = self.subscribers.lock();
        self.clear_subscribers(&mut subs);
        for sub in subs.iter_mut() {
            (sub.action)(val.clone());
        }
    }
}
