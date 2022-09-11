use gm::flat::Rect;
use rtools::Rglica;

use crate::{basic::RootView, view::ViewFrame, View};

pub(crate) trait ViewInternal {
    fn root_view(&self) -> Rglica<RootView>;
    fn super_absolute_frame(&self) -> &Rect;
}

impl<T: ?Sized + View> ViewInternal for T {
    fn root_view(&self) -> Rglica<RootView> {
        self.root_view
    }

    fn super_absolute_frame(&self) -> &Rect {
        if self.superview.is_ok() {
            return self.superview.absolute_frame();
        }
        self.absolute_frame()
    }
}
