use refs::Weak;

use crate::{View, ViewData};

pub(crate) struct TouchLayer {
    root:    Weak<dyn View>,
    touches: Vec<Weak<dyn View>>,
}

impl TouchLayer {
    pub(crate) fn add(&mut self, view: Weak<dyn View>, priority: bool) {
        self.touches.retain(Weak::is_ok);
        if priority {
            self.touches.insert(0, view);
        } else {
            self.touches.push(view);
        }
    }

    pub(crate) fn remove(&mut self, view: Weak<dyn View>) {
        self.touches.retain(|a| a.addr() != view.addr());
    }

    pub(crate) fn views(&self) -> Vec<Weak<dyn View>> {
        self.touches.clone()
    }

    pub(crate) fn root_addr(&self) -> usize {
        self.root.addr()
    }

    pub(crate) fn root_name(&self) -> &str {
        self.root.label()
    }
}

impl From<Weak<dyn View>> for TouchLayer {
    fn from(root: Weak<dyn View>) -> Self {
        Self {
            root,
            touches: vec![],
        }
    }
}
