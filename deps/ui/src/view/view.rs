use refs::{AsAny, Weak};

use crate::{ViewBase, ViewCallbacks, view::view_callbacks::__ViewInternalSetup};

#[allow(clippy::mut_from_ref)]
pub trait View: ViewCallbacks + __ViewInternalSetup + AsAny {
    fn init_views(&mut self);
    fn base_view(&self) -> &mut ViewBase;
    fn weak_view(&self) -> WeakView;
}

pub type WeakView = Weak<dyn View>;
