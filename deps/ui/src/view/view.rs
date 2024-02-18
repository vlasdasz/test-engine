use std::fmt::Debug;

use refs::{AsAny, Weak};

use crate::{view::view_callbacks::ViewInternalSetup, ViewBase, ViewCallbacks};

pub trait View: ViewCallbacks + ViewInternalSetup + AsAny + Debug {
    fn init_views(&mut self);
    fn base(&self) -> &ViewBase;
    fn base_mut(&mut self) -> &mut ViewBase;
    fn weak_view(&self) -> WeakView;
}

pub type WeakView = Weak<dyn View>;
