use crate::View;

pub trait CellCallbacks {
    fn cell_removed(&mut self);
}

impl<T: ?Sized + View> CellCallbacks for T {
    default fn cell_removed(&mut self) {}
}
