use gm::flat::Rect;
use rtools::Rglica;

use crate::{
    view::{ViewFrame, ViewSubviews},
    View,
};

pub(crate) trait ViewInternal {
    fn root_view(&self) -> Rglica<dyn View>;
    fn super_absolute_frame(&self) -> &Rect;
}

impl<T: ?Sized + View> ViewInternal for T {
    fn root_view(&self) -> Rglica<dyn View> {
        let mut root = self.superview();
        loop {
            if root.superview().is_null() {
                return root;
            }
            root = root.superview();
        }
    }

    fn super_absolute_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.absolute_frame();
        }
        self.absolute_frame()
    }
}
