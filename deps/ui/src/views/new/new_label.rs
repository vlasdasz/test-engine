use ui_proc::view;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct NewLabel<T: 'static> {
    #[educe(Debug(ignore))]
    value: T,
}

impl<T> NewLabel<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}
