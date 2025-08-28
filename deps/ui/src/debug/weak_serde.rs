use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use crate::WeakView;

static STR_STORAGE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct WeakRepr {
    ptr:       usize,
    stamp:     u64,
    type_name: &'static str,
}

pub(super) fn serialize_weak<S: Serializer>(
    name: &'static str,
    weak: WeakView,
    s: &mut S::SerializeStruct,
) -> Result<(), S::Error> {
    let (ptr, stamp, type_name) = weak.raw();

    s.serialize_field(
        name,
        &WeakRepr {
            ptr,
            stamp,
            type_name,
        },
    )
}

fn string_to_static(_string: String) -> &'static str {
    ""
}
