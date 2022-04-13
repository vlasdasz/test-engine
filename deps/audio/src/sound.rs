#![cfg(not(target_os = "android"))]

use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rtools::data_manager::LoadFromPath;

pub struct Sound {
    _path:         PathBuf,
    _stream:       OutputStream,
    stream_handle: OutputStreamHandle,
    data:          Vec<u8>,
}

impl Sound {
    pub fn play(&mut self) {
        let cursor = Cursor::new(self.data.clone());
        let input = Decoder::new(cursor).unwrap();
        self.stream_handle
            .play_raw(input.convert_samples())
            .unwrap();
    }
}

impl LoadFromPath for Sound {
    fn load(path: &Path) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let mut file =
            File::open(path).unwrap_or_else(|_| panic!("{}", path.to_string_lossy().to_string()));

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Self {
            _path: path.to_path_buf(),
            _stream,
            stream_handle,
            data,
        }
    }
}
