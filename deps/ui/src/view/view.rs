use std::fmt::Debug;

use rtools::Boxed;

use crate::{ViewBase, ViewCallbacks};

pub trait View: Boxed + Debug + ViewCallbacks {
    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}
