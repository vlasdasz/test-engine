use rtools::managed;

// mod android_sound;
// use android_sound as sound;
mod sound;

pub use sound::Sound;

managed!(Sound);
