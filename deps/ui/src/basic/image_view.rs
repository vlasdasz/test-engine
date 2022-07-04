use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{impl_view, view, View, ViewBase, ViewCallbacks, ViewData};

#[view]
#[derive(Default, Debug)]
pub struct ImageView {}
impl_view!(ImageView);

impl ViewCallbacks for ImageView {
    fn setup(&mut self) {
        self.set_color(Color::CLEAR);
    }
}
