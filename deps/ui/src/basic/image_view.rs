use crate::{View, ViewBase};

#[derive(Default, Debug)]
pub struct ImageView {
    base: ViewBase,
}

impl View for ImageView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
