use std::{fs::File, io::BufReader, path::Path};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rtools::data_manager::LoadFromPath;

pub struct Sound {
    source: Decoder<BufReader<File>>,
    stream: OutputStreamHandle,
}

impl Sound {
    pub fn play(&self) {
        //_ = self.stream.play_raw(self.source.convert_samples());
    }
}

impl LoadFromPath for Sound {
    fn load(path: &Path) -> Self {
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device

        todo!()
    }
}
