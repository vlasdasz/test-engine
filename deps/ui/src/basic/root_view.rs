use rtools::{address::Address, Boxed, Rglica, ToRglica};

use crate::{impl_view, view, view::ViewSubviews, View, ViewBase};

#[view]
#[derive(Default, Debug)]
pub struct RootView {
    to_remove: Vec<Rglica<dyn View>>,
}
impl_view!(RootView);

impl RootView {
    pub fn make_root() -> Box<Self> {
        let mut root = Self::boxed();
        let rg = root.to_rglica();
        root.view_mut().root_view = rg;
        root
    }

    pub fn remove_sheduled(&mut self) {
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

    pub(crate) fn shedule_remove(&mut self, view: Rglica<dyn View>) {
        self.to_remove.push(view)
    }
}
