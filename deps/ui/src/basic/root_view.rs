use rtools::{address::Address, Boxed, Rglica, ToRglica};

use crate::{test::debug_view::DebugView, view, view::ViewSubviews, SubView, View, ViewBase};

#[view]
#[derive(Default)]
pub struct RootView {
    pub debug_view: SubView<DebugView>,
    to_remove:      Vec<Rglica<dyn View>>,
}

impl RootView {
    pub fn new() -> Box<Self> {
        let mut root = Self::boxed();
        let rg = root.to_rglica();
        root.root_view = rg;
        root.debug_view = root.add_view();
        root
    }

    pub fn remove_scheduled(&mut self) {
        if self.to_remove.is_empty() {
            return;
        }
        let to_remove = self.to_remove.drain(..);
        for view in to_remove {
            let index = view
                .superview()
                .subviews()
                .iter()
                .position(|sub| view.address() == sub.address())
                .unwrap();
            view.superview().remove_subview_at(index);
        }
    }

    pub(crate) fn schedule_remove(&mut self, view: Rglica<dyn View>) {
        self.to_remove.push(view)
    }
}
