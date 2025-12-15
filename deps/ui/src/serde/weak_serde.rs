use std::{
    collections::HashSet,
    mem::transmute,
    sync::{Mutex, OnceLock},
};

use refs::RawPointer;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use crate::WeakView;

#[derive(Serialize, Deserialize)]
pub(super) struct WeakRepr {
    addr:      usize,
    stamp:     u64,
    type_name: String,
}

pub(super) fn serialize_weak<S: Serializer>(
    name: &'static str,
    weak: WeakView,
    s: &mut S::SerializeStruct,
) -> Result<(), S::Error> {
    let raw = weak.raw();

    s.serialize_field(
        name,
        &WeakRepr {
            addr:      raw.addr(),
            stamp:     raw.stamp(),
            type_name: raw.type_name().to_string(),
        },
    )
}

pub(super) fn deserialize_weak(value: WeakRepr) -> WeakView {
    unsafe {
        WeakView::from_raw(RawPointer::new(
            value.addr,
            value.stamp,
            string_to_static(value.type_name),
        ))
    }
}

fn string_to_static(string: String) -> &'static str {
    static STR_STORAGE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

    let mut storage = STR_STORAGE.get_or_init(|| Mutex::new(HashSet::new())).lock().unwrap();

    storage.insert(string.clone());

    let result = storage.get(&string).unwrap().as_str();

    unsafe { transmute(result) }
}
