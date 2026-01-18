use inspect::ui::ViewRepr;
use test_engine::{
    refs::Weak,
    ui::{ViewSubviews, view},
};

use crate::ui::inspect::view_view::ViewView;

#[view]
pub struct UIRepresentView {
    scale: f32,
    repr:  ViewRepr,
}

impl UIRepresentView {
    pub fn reload(self: Weak<Self>) {
        self.set_root(self.scale, self.repr.clone());
    }

    pub fn set_root(mut self: Weak<Self>, scale: f32, repr: ViewRepr) {
        self.remove_all_subviews();
        let view = self.add_view::<ViewView>();
        view.set_repr(scale, repr.clone());
        self.scale = scale;
        self.repr = repr;
    }
}
