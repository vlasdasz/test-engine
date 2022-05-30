use std::ops::DerefMut;

use rtools::{Dispatch, ToRglica};
use serde::de::DeserializeOwned;

use crate::Request;

pub struct DispatchRequest<Param, Result> {
    request: Request<Param, Result>,
}

impl<R, P> DispatchRequest<R, P> {
    pub const fn make(base: &'static str, url: &'static str) -> Self {
        Self {
            request: Request::make(base, url),
        }
    }
}

impl<Result: DeserializeOwned + Default + Sync + Send> DispatchRequest<(), Result> {
    pub fn get<Obj: 'static>(
        &'static self,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<String>, Result) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.get(), move |result| match result {
            Ok(val) => completion(rglica.deref_mut(), None, val),
            Err(_) => completion(
                rglica.deref_mut(),
                Some("Request error".into()),
                Result::default(),
            ),
        });
    }
}
