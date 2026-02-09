use crate::View;

pub(crate) trait ViewTouchInternal {
    fn touch_id(&self) -> u64;
    fn set_touch_id(&mut self, id: u64) -> &mut Self;
}

impl<T: ?Sized + View> ViewTouchInternal for T {
    fn touch_id(&self) -> u64 {
        self.__base_view().touch_id
    }

    fn set_touch_id(&mut self, id: u64) -> &mut Self {
        self.__base_view().touch_id = id;
        self
    }
}
