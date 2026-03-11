use netrun::Function;
use parking_lot::Mutex;
use refs::Weak;

struct Subscriber<T: Send + Clone> {
    subscriber: Weak,
    action:     Function<T, ()>,
}

pub struct UIEvent<T: Send + Clone = ()> {
    subscribers: Mutex<Vec<Subscriber<T>>>,
}

impl<T: Send + Clone> Default for UIEvent<T> {
    fn default() -> Self {
        Self {
            subscribers: Mutex::default(),
        }
    }
}

impl<T: Send + Clone> UIEvent<T> {
    pub const fn const_new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
        }
    }

    pub fn clear_subscribers(&self) -> &Self {
        self.subscribers.lock().clear();
        self
    }

    pub fn sub<U: ?Sized>(&self, subscriber: Weak<U>, mut action: impl FnMut() + Send + 'static) {
        self.val(subscriber, move |_| action());
    }

    pub fn val<U: ?Sized>(&self, subscriber: Weak<U>, action: impl FnMut(T) + Send + 'static) {
        let mut subs = self.subscribers.lock();

        // Remove if this view is already subscribed
        subs.retain(|s| s.subscriber.raw() != subscriber.raw());

        subs.push(Subscriber {
            subscriber: subscriber.erase(),
            action:     Function::new(action),
        });
    }

    pub fn unsubscribe<U: ?Sized>(&self, view: Weak<U>) {
        self.subscribers.lock().retain(|s| s.subscriber.raw() != view.raw());
    }

    pub fn trigger(&self, val: T)
    where T: Clone {
        let actions: Vec<_> = self.subscribers.lock().iter().map(|s| s.action.clone()).collect();

        for action in actions {
            action.call(val.clone())
        }
    }
}
