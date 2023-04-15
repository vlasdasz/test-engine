// #![cfg(not_android)]

use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rtools::data_manager::ResourceLoader;

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
        self.stream_handle.play_raw(input.convert_samples()).unwrap();
    }
}

impl ResourceLoader for Sound {
    fn load_path(path: &Path) -> Self {
        let mut file = File::open(path).unwrap_or_else(|_| panic!("{}", path.to_string_lossy().to_string()));

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Self::load_data(&data, path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            _path: name.to_string().into(),
            _stream: stream,
            stream_handle,
            data: data.into(),
        }
    }
}
