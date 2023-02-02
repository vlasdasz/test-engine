use cfg_if::cfg_if;
use rtools::{
    data_manager::{DataManager, DataStorage, Managed},
    managed,
};

cfg_if! { if #[cfg(android)] {
    // mod android_sound;
    // use android_sound as sound;
    mod sound;
} else {
    mod sound;
}}

pub use sound::Sound;

managed!(Sound);
