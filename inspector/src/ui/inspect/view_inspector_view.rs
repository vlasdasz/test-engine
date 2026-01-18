use inspect::ui::ViewRepr;
use test_engine::{
    refs::Weak,
    ui::{HasText, Label, Setup, ViewData, view},
};

use crate::ui::inspect::placer_view::PlacerView;

#[view]
pub struct ViewInspectorView {
    view: ViewRepr,

    #[init]
    label:       Label,
    id:          Label,
    placer_view: PlacerView,
}

impl Setup for ViewInspectorView {
    fn setup(self: Weak<Self>) {
        self.place().all_ver();
    }
}

impl ViewInspectorView {
    pub fn set_view(mut self: Weak<Self>, view: ViewRepr) {
        dbg!(&view);
        self.label.set_text(format!("Label: {}", view.label));
        self.id.set_text_size(10).set_text(format!("{}", view.id));

        self.view = view;
    }
}
