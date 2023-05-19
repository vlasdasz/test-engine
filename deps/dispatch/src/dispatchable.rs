use std::ops::Deref;

use refs::Weak;

use crate::from_main;

pub trait Dispatchable<T: Clone + Send> {
    async fn get_async(self) -> T;
    async fn set_async(self, val: T);
    async fn edit_async(self, edit: impl FnOnce(&mut T) + Send + 'static);
}

impl<T: Clone + Send + 'static> Dispatchable<T> for Weak<T> {
    async fn get_async(self) -> T {
        from_main(move || self.deref().clone()).await
    }

    async fn set_async(mut self, val: T) {
        from_main(move || *self = val).await;
    }

    async fn edit_async(mut self, edit: impl FnOnce(&mut T) + Send + 'static) {
        from_main(move || {
            let mut val: T = self.deref().clone();
            edit(&mut val);
            *self = val;
        })
        .await;
    }
}
