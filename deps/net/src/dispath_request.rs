use std::ops::DerefMut;

use rtools::{Dispatch, ToRglica};
use serde::de::DeserializeOwned;

use crate::Request;

pub struct DispatchRequest<Param, Result> {
    request: Request<Param, Result>,
}

impl<R, P> DispatchRequest<R, P> {
    pub const fn make(url: &'static str) -> Self {
        Self {
            request: Request::make(url),
        }
    }
}

// pub fn set<Obj: 'static>(&self, obj: &Obj, mut action: impl FnMut(&mut Obj,
// T) + 'static) {     debug_assert!(
//         self.subscriber.borrow().is_null(),
//         "Event already has a subscriber"
//     );
//     let mut rglica = obj.to_rglica();
//     self.subscriber
//         .replace(Unwrap::from_box(Box::new(move |value| {
//             action(rglica.deref_mut(), value);
//         })));
// }

impl<Result: DeserializeOwned + Default + Sync + Send> DispatchRequest<(), Result> {
    pub fn get<Obj: 'static>(
        &'static self,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<String>, Result) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.get(), move |result| {
            if result.is_err() {
                completion(
                    rglica.deref_mut(),
                    Some("Request error".into()),
                    Result::default(),
                )
            } else {
                completion(rglica.deref_mut(), None, result.unwrap())
            }
        });
    }
}
