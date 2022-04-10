use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use rodio::{OutputStream, OutputStreamHandle, Sink};
use rtools::{data_manager::LoadFromPath};

pub struct Sound {
    _path:          PathBuf,
    _stream:        OutputStream,
    _stream_handle: OutputStreamHandle,
    sink:          Sink,
}

impl Sound {
    pub fn play(&self) {
        self.sink.play()
    }
}

impl LoadFromPath for Sound {
    fn load(path: &Path) -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(path).unwrap());
        let sink = _stream_handle.play_once(file).unwrap();
        sink.pause();

        Self {
            _path: path.to_path_buf(),
            _stream,
            _stream_handle,
            sink,
        }
    }
}
