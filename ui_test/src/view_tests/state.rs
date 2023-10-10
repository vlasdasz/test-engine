use std::sync::{Mutex, MutexGuard, OnceLock};

static STATE: OnceLock<State> = OnceLock::new();

#[derive(Default)]
struct State {
    val: Mutex<u32>,
}

fn get() -> MutexGuard<'static, u32> {
    STATE.get_or_init(Default::default).val.lock().unwrap()
}

pub fn clear_state() {
    *get() = 0;
}

pub fn get_state() -> u32 {
    *get()
}

pub fn increment_state() {
    *get() += 1;
}

#[cfg(test)]
mod test {
    use crate::view_tests::state::{clear_state, get_state, increment_state};

    #[test]
    fn test_state() {
        assert_eq!(get_state(), 0);
        increment_state();
        increment_state();
        increment_state();
        assert_eq!(get_state(), 3);
        clear_state();
        assert_eq!(get_state(), 0);
    }
}
