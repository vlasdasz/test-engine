use refs::Weak;

use crate::{ViewData, WeakView};

pub(crate) struct TouchLayer {
    pub(crate) root: WeakView,
    touches:         Vec<WeakView>,
}

impl TouchLayer {
    pub(crate) fn add(&mut self, view: WeakView) {
        self.touches.push(view);
    }

    pub(crate) fn remove(&mut self, view: WeakView) {
        self.touches.retain(|a| a.addr() != view.addr());
    }

    pub(crate) fn views(&self) -> &[WeakView] {
        &self.touches
    }

    pub(crate) fn root_addr(&self) -> usize {
        self.root.addr()
    }

    pub(crate) fn root_name(&self) -> &str {
        self.root.label()
    }

    pub(crate) fn clear_freed(&mut self) {
        assert!(self.root.is_ok());
        self.touches.retain(Weak::is_ok);
    }
}

impl From<WeakView> for TouchLayer {
    fn from(root: WeakView) -> Self {
        Self {
            root,
            touches: vec![],
        }
    }
}
