use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rtools::data_manager::LoadFromPath;

pub struct Sound {
    _path:          PathBuf,
    _stream:        OutputStream,
    _stream_handle: OutputStreamHandle,
    data:           Vec<u8>,
    sink:           Sink,
}

impl Sound {
    pub fn play(&self) {
        let cursor = Cursor::new(self.data.clone());
        let input = Decoder::new(cursor).unwrap();
        self.sink.append(input);
        self.sink.play()
    }
}

impl LoadFromPath for Sound {
    fn load(path: &Path) -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();

        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let cursor = Cursor::new(data.clone());
        let sink = _stream_handle.play_once(cursor).unwrap();

        Self {
            _path: path.to_path_buf(),
            _stream,
            _stream_handle,
            data,
            sink,
        }
    }
}
