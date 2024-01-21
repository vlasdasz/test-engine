// #![cfg(not_android)]

use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use manage::resource_loader::ResourceLoader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Sound {
    path:          PathBuf,
    _stream:       OutputStream,
    stream_handle: OutputStreamHandle,
    data:          Vec<u8>,
}

impl Sound {
    pub fn play(&mut self) {
        let cursor = Cursor::new(self.data.clone());
        let input = Decoder::new(cursor).unwrap();

        let sink = Sink::try_new(&self.stream_handle).unwrap();

        sink.set_volume(0.1);
        sink.append(input);
        sink.detach();
    }
}

impl ResourceLoader for Sound {
    fn load_path(path: &Path) -> Self {
        let mut file = File::open(path)
            .unwrap_or_else(|_| panic!("Failed to load sound at path: {}", path.to_string_lossy()));

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Self::load_data(&data, path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            path: name.to_string().into(),
            _stream: stream,
            stream_handle,
            data: data.into(),
        }
    }
}

impl Debug for Sound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}
