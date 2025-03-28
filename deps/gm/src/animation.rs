use chrono::Utc;

use crate::LossyConvert;

const SEC: f32 = 1_000.0;

#[derive(Default, Debug)]
pub struct Animation {
    start:    f32,
    span:     f32,
    duration: f32,
    stamp:    i64,
}

impl Animation {
    pub fn new(start: impl Into<f32>, end: impl Into<f32>, duration: impl Into<f32>) -> Self {
        let start = start.into() * SEC;
        let end = end.into() * SEC;
        let span = end - start;
        assert_ne!(span.to_bits(), 0);
        Self {
            start,
            span,
            duration: duration.into() * SEC,
            stamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn finished(&self) -> bool {
        Utc::now().timestamp_millis() >= self.stamp + LossyConvert::<i64>::lossy_convert(self.duration)
    }

    pub fn value(&self) -> f32 {
        let now = Utc::now().timestamp_millis();
        let delta: f32 = (now - self.stamp).lossy_convert();
        let passed: u64 = (delta / self.duration).lossy_convert();
        let even = passed % 2 == 0;
        let passed: f32 = passed.lossy_convert();
        let delta = delta - (passed * self.duration);
        let ratio = delta / (self.duration);
        let span = if even {
            self.span * ratio
        } else {
            self.span - self.span * ratio
        };
        (self.start + span) / SEC
    }
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use crate::Animation;

    #[test]
    fn test() {
        let anim = Animation::new(0.0, 1.0, 0.5);

        assert!(
            anim.value() >= 0.0 && anim.value() <= 0.002,
            "Actual: {}",
            anim.value()
        );
        assert_eq!(anim.finished(), false);

        sleep(Duration::from_secs_f32(0.25));

        assert_eq!(anim.finished(), false);
        assert!(
            anim.value() >= 0.48 && anim.value() <= 0.52,
            "Actual: {}",
            anim.value()
        );

        sleep(Duration::from_secs_f32(0.10));

        assert_eq!(anim.finished(), false);
        assert!(
            anim.value() >= 0.70 && anim.value() <= 0.74,
            "Actual: {}",
            anim.value()
        );

        sleep(Duration::from_secs_f32(0.15));

        assert_eq!(anim.finished(), true);
        assert!(
            anim.value() >= 0.92 && anim.value() <= 1.04,
            "Actual: {}",
            anim.value()
        );

        sleep(Duration::from_secs_f32(0.25));

        assert_eq!(anim.finished(), true);
        assert!(
            anim.value() >= 0.40 && anim.value() <= 0.60,
            "Actual: {}",
            anim.value()
        );
    }
}
