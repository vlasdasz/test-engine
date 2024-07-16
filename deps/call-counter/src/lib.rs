use std::{collections::BTreeMap, sync::Mutex};

static COUNTER: Mutex<BTreeMap<u32, u32>> = Mutex::new(BTreeMap::new());

#[macro_export]
macro_rules! count_calls {
    () => {
        call_counter::__increment_counter(file!(), line!());
    };
}

pub fn __increment_counter(file: &'static str, line: u32) {
    let mut lock = COUNTER.lock().unwrap();
    let counter = lock.entry(line).or_default();

    *counter += 1;

    println!("{}:{line} - {counter}", file);
}

#[cfg(test)]
mod test {

    use crate as call_counter;

    fn call() {
        count_calls!();
    }

    #[test]
    fn test() {
        for _ in 0..10 {
            call();
        }
    }
}
