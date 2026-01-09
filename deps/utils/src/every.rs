use parking_lot::Mutex;
use web_time::Instant;

static LAST_UPDATE: Mutex<Option<Instant>> = Mutex::new(None);

pub struct Every {}

impl Every {
    pub fn second(action: impl FnOnce()) {
        let mut lock = LAST_UPDATE.lock();

        let last_update = lock.unwrap_or_else(|| {
            let now = Instant::now();
            *lock = now.into();
            now
        });

        if last_update.elapsed().as_secs_f32() < 1.0 {
            return;
        }

        action();

        *lock = Instant::now().into();
    }
}
