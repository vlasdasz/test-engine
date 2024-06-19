use std::sync::Mutex;

use refs::Weak;

use crate::WeakView;

struct Subscriber<T: Send> {
    subscriber: Weak,
    action:     Box<dyn FnMut(T) + Send>,
}

#[derive(Default)]
pub struct UIEvent<T: Send = ()> {
    subscribers: Mutex<Vec<Subscriber<T>>>,
}

impl<T: Send> UIEvent<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(vec![]),
        }
    }

    pub fn sub<U: ?Sized>(&self, subscriber: Weak<U>, mut action: impl FnMut() + Send + 'static) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.subscriber.is_ok());

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
        subs.retain(|a| a.subscriber.is_ok());

        assert!(
            !subs.iter().any(|s| s.subscriber.addr() == subscriber.addr()),
            "This object is already subscribed to this event"
        );

        subs.push(Subscriber {
            subscriber: subscriber.erase(),
            action:     Box::new(action),
        });
    }

    pub fn unsibscribe(&self, view: WeakView) {
        self.subscribers.lock().unwrap().retain(|a| a.subscriber.addr() != view.addr());
    }

    pub fn trigger(&self, val: T)
    where T: Clone {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.subscriber.is_ok());
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
