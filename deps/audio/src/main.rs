use std::path::Path;

use audio::Sound;
use rtools::data_manager::LoadFromPath;

fn main() {
    let sound = Sound::load(Path::new(
        "/Users/vladas/Downloads/Electrochok - Adam Park.mp3",
    ));
    sound.play();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
