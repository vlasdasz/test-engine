use std::sync::{Mutex, MutexGuard};

use refs::Weak;

struct Subscriber<T: Send> {
    subscriber: Weak,
    action:     Box<dyn FnMut(T) + Send>,
}

pub struct UIEvent<T: Send = ()> {
    subscribers: Mutex<Vec<Subscriber<T>>>,
    /// This allows unsibscribing from the event during its execution
    unsubscribe: Mutex<Vec<usize>>,
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
    fn clear_subscribers(&self, subs: &mut MutexGuard<Vec<Subscriber<T>>>) {
        let mut unsubscribe = self.unsubscribe.lock().unwrap();
        subs.retain(|a| !unsubscribe.contains(&a.subscriber.addr()) && a.subscriber.is_ok());
        unsubscribe.clear();
    }

    pub fn sub<U: ?Sized>(&self, subscriber: Weak<U>, mut action: impl FnMut() + Send + 'static) {
        let mut subs = self.subscribers.lock().unwrap();
        self.clear_subscribers(&mut subs);

        assert!(
            !subs.iter().any(|s| s.subscriber.addr() == subscriber.addr()),
            "This object is already subscribed to this event"
        );

        subs.push(Subscriber {
            subscriber: subscriber.erase(),
            action:     Box::new(move |_| action()),
        });
    }

    pub fn val<U: ?Sized>(&self, subscriber: Weak<U>, action: impl FnMut(T) + Send + 'static) {
        let mut subs = self.subscribers.lock().unwrap();
        self.clear_subscribers(&mut subs);

        assert!(
            !subs.iter().any(|s| s.subscriber.addr() == subscriber.addr()),
            "This object is already subscribed to this event"
        );

        subs.push(Subscriber {
            subscriber: subscriber.erase(),
            action:     Box::new(action),
        });
    }

    pub fn unsibscribe<U: ?Sized>(&self, view: Weak<U>) {
        self.unsubscribe.lock().unwrap().push(view.addr());
    }

    pub fn trigger(&self, val: T)
    where T: Clone {
        let mut subs = self.subscribers.lock().unwrap();
        self.clear_subscribers(&mut subs);
        for sub in subs.iter_mut() {
            (sub.action)(val.clone());
        }
    }

    // pub fn dump_subscribers(&self) -> Vec<String> {
    //     let mut subs = self.subscribers.lock().unwrap();
    //     subs.retain(|a| a.subscriber.is_ok());
    //     subs.iter()
    //         .map(|s| format!("{} - {}", s.subscriber.label(),
    // s.subscriber.addr()))         .collect()
    // }
}
