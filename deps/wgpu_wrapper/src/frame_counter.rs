use gm::LossyConvert;

pub(crate) struct FrameCounter {
    // Instant of the last time we printed the frame time.
    last_printed_instant: web_time::Instant,
    // Number of frames since the last time we printed the frame time.
    frame_count:          u32,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            last_printed_instant: web_time::Instant::now(),
            frame_count:          0,
        }
    }
}

impl FrameCounter {
    pub fn update(&mut self) -> Option<(f32, f32)> {
        self.frame_count += 1;
        let new_instant = web_time::Instant::now();
        let elapsed_secs = (new_instant - self.last_printed_instant).as_secs_f32();

        if elapsed_secs < 0.2 {
            return None;
        }

        let elapsed_ms = elapsed_secs;
        let frame_time = elapsed_ms / self.frame_count.lossy_convert();
        let fps = self.frame_count.lossy_convert() / elapsed_secs;

        self.last_printed_instant = new_instant;
        self.frame_count = 0;

        Some((frame_time, fps))
    }
}
