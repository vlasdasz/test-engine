use rtools::{Rglica, ToRglica};

use crate::{impl_view, view, view::ViewSubviews, Label, View, ViewBase, ViewCallbacks};

#[view]
#[derive(Default, Debug)]
pub struct TextField {
    label: Rglica<Label>,
}
impl_view!(TextField);

impl ViewCallbacks for TextField {
    fn setup(&mut self) {
        self.label = self.add_view();
    }
}
