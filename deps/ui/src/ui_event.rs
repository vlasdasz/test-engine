use std::{ops::Deref, sync::Mutex};

use crate::{View, ViewData, WeakView};

struct Subscriber<T: Send> {
    view:   WeakView,
    action: Box<dyn FnMut(T) + Send>,
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

    pub fn sub(
        &self,
        view: impl Deref<Target = impl View + ?Sized>,
        mut action: impl FnMut() + Send + 'static,
    ) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.view.is_ok());

        let view = view.weak_view();

        assert!(
            !subs.iter().any(|s| s.view.addr() == view.addr()),
            "This view is already subscribed to this event"
        );

        subs.push(Subscriber {
            view,
            action: Box::new(move |_| action()),
        })
    }

    pub fn val(&self, view: impl Deref<Target = impl View + ?Sized>, action: impl FnMut(T) + Send + 'static) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.view.is_ok());

        let view = view.weak_view();

        assert!(
            !subs.iter().any(|s| s.view.addr() == view.addr()),
            "This view is already subscribed to this event"
        );

        subs.push(Subscriber {
            view:   view.weak_view(),
            action: Box::new(action),
        })
    }

    pub fn unsibscribe(&self, view: WeakView) {
        self.subscribers.lock().unwrap().retain(|a| a.view.addr() != view.addr())
    }

    pub fn trigger(&self, val: T)
    where T: Clone {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.view.is_ok());
        for sub in subs.iter_mut() {
            (sub.action)(val.clone())
        }
    }

    pub fn dump_subscribers(&self) -> Vec<String> {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.view.is_ok());
        subs.iter().map(|s| format!("{} - {}", s.view.label(), s.view.addr())).collect()
    }
}
