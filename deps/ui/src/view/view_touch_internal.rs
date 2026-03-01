use crate::View;

const NO_TOUCH: usize = 0;

pub(crate) trait ViewTouchInternal {
    fn touch_id(&self) -> usize;
    fn set_touch_id(&mut self, id: usize) -> &mut Self;
    fn reset_touch_id(&mut self);
}

impl<T: ?Sized + View> ViewTouchInternal for T {
    fn touch_id(&self) -> usize {
        self.__base_view().touch_id
    }

    fn set_touch_id(&mut self, id: usize) -> &mut Self {
        self.__base_view().touch_id = id;
        self
    }

    fn reset_touch_id(&mut self) {
        self.set_touch_id(NO_TOUCH);
    }
}
