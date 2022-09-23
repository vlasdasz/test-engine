use std::{borrow::Borrow, ops::DerefMut};

use rtools::{Dispatch, ToRglica};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Request};

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

impl<Result: DeserializeOwned + Default + Sync + Send> DispatchRequest<(), Result> {
    pub fn get<Obj: 'static>(
        &'static self,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<Error>, Result) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.get(), move |result| match result {
            Ok(val) => completion(rglica.deref_mut(), None, val),
            Err(err) => completion(rglica.deref_mut(), err.into(), Result::default()),
        });
    }
}

impl<Param: Serialize> DispatchRequest<Param, ()> {
    pub fn post<Obj: 'static>(
        &'static self,
        param: impl Borrow<Param> + Send + 'static,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<Error>) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.post(param), move |result| match result {
            Ok(_) => completion(rglica.deref_mut(), None),
            Err(err) => completion(rglica.deref_mut(), err.into()),
        });
    }
}

impl<Param, Result> DispatchRequest<Param, Result>
where
    Param: Serialize,
    Result: DeserializeOwned + Default + Sync + Send,
{
    pub fn fetch<Obj: 'static>(
        &'static self,
        param: impl Borrow<Param> + Send + 'static,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<Error>, Result) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.fetch(param), move |response| match response {
            Ok(val) => completion(rglica.deref_mut(), None, val),
            Err(err) => completion(rglica.deref_mut(), err.into(), Result::default()),
        });
    }
}

pub type GetRequest<T> = DispatchRequest<(), T>;
pub type PostRequest<T> = DispatchRequest<T, ()>;
