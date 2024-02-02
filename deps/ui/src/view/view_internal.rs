use gm::flat::Rect;

use crate::{
    view::{view_subviews::ViewSubviews, ViewFrame},
    View,
};

pub(crate) trait ViewInternal {
    fn super_absolute_frame(&self) -> &Rect;
}

impl<T: ?Sized + View> ViewInternal for T {
    fn super_absolute_frame(&self) -> &Rect {
        if self.superview().is_ok() {
            return self.base().superview.absolute_frame();
        }
        self.absolute_frame()
    }
}
