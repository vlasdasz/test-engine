use std::{borrow::Borrow, ops::DerefMut};

use rtools::{Dispatch, ToRglica};
use serde::{de::DeserializeOwned, Serialize};

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
            Err(err) => completion(rglica.deref_mut(), error_to_string(err), Result::default()),
        });
    }
}

impl<Param: Serialize> DispatchRequest<Param, ()> {
    pub fn post<Obj: 'static>(
        &'static self,
        param: impl Borrow<Param> + Send + 'static,
        obj: &Obj,
        completion: impl FnOnce(&mut Obj, Option<String>) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        Dispatch::dispatch(self.request.post(param), move |result| match result {
            Ok(_) => completion(rglica.deref_mut(), None),
            Err(err) => completion(rglica.deref_mut(), error_to_string(err)),
        });
    }
}

fn error_to_string(error: impl Borrow<reqwest::Error>) -> Option<String> {
    let error = error.borrow();
    // dbg!(error);
    //
    // dbg!(error.is_connect());
    // dbg!(error.is_request());
    // dbg!(error.is_redirect());
    // dbg!(error.is_decode());
    // dbg!(error.is_status());
    // dbg!(error.status());
    //
    // println!("{}", error);

    Some(format!("{}", error))
}

pub type GetRequest<T> = DispatchRequest<(), T>;
pub type PostRequest<T> = DispatchRequest<T, ()>;
