use crate::View;

pub(crate) trait ViewTouchInternal {
    fn touch_id(&self) -> u64;
    fn set_touch_id(&mut self, id: u64) -> &mut Self;
    fn touch_enabled(&self) -> bool;
}

impl<T: ?Sized + View> ViewTouchInternal for T {
    fn touch_id(&self) -> u64 {
        self.touch_id
    }

    fn set_touch_id(&mut self, id: u64) -> &mut Self {
        self.touch_id = id;
        self
    }

    fn touch_enabled(&self) -> bool {
        *self.touch_enabled.borrow()
    }
}
