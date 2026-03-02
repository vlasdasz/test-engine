use refs::{
    Weak,
    vec::{WeakVec, WeakVecHelper},
};

use crate::{ScrollView, ViewData, WeakView};

pub(crate) struct TouchLayer {
    pub(crate) root: WeakView,
    listeners:       Vec<WeakView>,
    scrolls:         WeakVec<ScrollView>,
}

impl TouchLayer {
    pub(crate) fn add_scroll(&mut self, view: Weak<ScrollView>) {
        self.scrolls.push(view);
    }

    pub(crate) fn add(&mut self, view: WeakView) {
        self.listeners.push(view);
    }

    pub(crate) fn add_low_priority(&mut self, view: WeakView) {
        self.listeners.insert(0, view);
    }

    pub(crate) fn remove(&mut self, view: WeakView) {
        self.listeners.retain(|a| a.raw() != view.raw());
    }

    pub(crate) fn views(&self) -> Vec<WeakView> {
        self.listeners.clone()
    }

    pub(crate) fn scrolls(&self) -> WeakVec<ScrollView> {
        self.scrolls.clone()
    }

    pub(crate) fn root_name(&self) -> &str {
        self.root.label()
    }

    pub(crate) fn clear_freed(&mut self) {
        assert!(self.root.is_ok());
        self.listeners.remove_freed();
        self.scrolls.remove_freed();
    }
}

impl From<WeakView> for TouchLayer {
    fn from(root: WeakView) -> Self {
        Self {
            root,
            listeners: vec![],
            scrolls: vec![],
        }
    }
}
