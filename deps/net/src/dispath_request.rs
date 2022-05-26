use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::Request;

pub struct DispatchRequest<Param, Result> {
    request: Request<Param, Result>,
}

impl<R, P> DispatchRequest<R, P> {
    pub fn make(url: impl ToString) -> Self {
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

impl<Result: DeserializeOwned + Default> DispatchRequest<(), Result> {
    pub async fn get<Obj: 'static>(&self, obj: &Obj, completion: impl Fn(&mut Obj, Option<String>, Result)) {


        todo!()
    }
}
