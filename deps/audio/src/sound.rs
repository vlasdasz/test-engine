use std::{
    default::Default,
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rtools::{data_manager::LoadFromPath, Unwrap};

pub struct Sound {
    _path:         PathBuf,
    _stream:       OutputStream,
    stream_handle: OutputStreamHandle,
    data:          Vec<u8>,
    sink:          Unwrap<Sink>,
}

impl Sound {
    pub fn play(&mut self) {
        let cursor = Cursor::new(self.data.clone());
        if self.sink.is_null() {
            self.sink = self.stream_handle.play_once(cursor).unwrap().into();
        } else {
            let input = Decoder::new(cursor).unwrap();
            self.sink.append(input);
            self.sink.play()
        }
    }
}

impl LoadFromPath for Sound {
    fn load(path: &Path) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Self {
            _path: path.to_path_buf(),
            _stream,
            stream_handle,
            data,
            sink: Default::default(),
        }
    }
}
