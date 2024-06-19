use web_time::Instant;

pub struct Elapsed {
    location: &'static str,
    start:    Instant,
}

impl Elapsed {
    pub fn start(location: &'static str) -> Self {
        Self {
            location,
            start: Instant::now(),
        }
    }
}

impl Drop for Elapsed {
    fn drop(&mut self) {
        println!("{} - {:?}", self.location, self.start.elapsed());
    }
}

#[macro_export]
macro_rules! elapsed {
    ($name:expr) => {
        let _a = Elapsed::start($name);
    };
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use crate::elapsed::Elapsed;

    #[test]
    fn test_elapsed() {
        elapsed!("a");
        sleep(Duration::from_secs_f32(0.1));
    }
}
