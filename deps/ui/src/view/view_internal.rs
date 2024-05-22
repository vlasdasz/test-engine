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
        let sup = self.superview();

        // Since superview owns subview this should be fine I hope.
        if sup.is_ok() {
            return unsafe { sup.deref_unchecked().absolute_frame() };
        }

        self.absolute_frame()
    }
}
