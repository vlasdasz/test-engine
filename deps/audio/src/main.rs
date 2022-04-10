use std::path::Path;

use audio::Sound;
use rtools::data_manager::LoadFromPath;

fn main() {
    let _sound = Sound::load(Path::new(
        "/Users/vladas/Downloads/mixkit-fast-small-sweep-transition-166.wav",
    ));

    std::thread::sleep(std::time::Duration::from_secs(1));
    _sound.play();
    std::thread::sleep(std::time::Duration::from_secs(1));
    _sound.play();
    std::thread::sleep(std::time::Duration::from_secs(1));
    _sound.play();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
