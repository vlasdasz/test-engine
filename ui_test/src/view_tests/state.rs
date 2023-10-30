use std::sync::{Mutex, MutexGuard, OnceLock};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

static STATE: OnceLock<State> = OnceLock::new();

#[derive(Default)]
struct State {
    val: Mutex<String>,
}

fn get() -> MutexGuard<'static, String> {
    STATE.get_or_init(Default::default).val.lock().unwrap()
}

pub fn clear_state() {
    *get() = String::default();
}

pub fn get_state<T: DeserializeOwned + Default>() -> T {
    from_str(&*get()).unwrap_or_default()
}

pub fn set_state<T: Serialize>(val: T) {
    *get() = to_string(&val).unwrap();
}

pub fn increment_state() {
    let val: u32 = get_state();
    set_state(val + 1);
}

#[cfg(test)]
mod test {
    use crate::view_tests::state::{clear_state, get_state, increment_state};

    #[test]
    fn test_state() {
        assert_eq!(get_state::<u32>(), 0);
        increment_state();
        increment_state();
        increment_state();
        assert_eq!(get_state::<u32>(), 3);
        clear_state();
        assert_eq!(get_state::<u32>(), 0);
        increment_state();
        assert_eq!(get_state::<u32>(), 1);
    }
}
