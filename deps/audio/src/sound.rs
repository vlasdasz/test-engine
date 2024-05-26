// #![cfg(not_android)]

use std::{
    fmt::{Debug, Formatter},
    fs::read,
    io::Cursor,
    path::{Path, PathBuf},
};

use log::error;
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

static DEFAULT_SOUND_DATA: &[u8] = include_bytes!("pek.wav");

impl ResourceLoader for Sound {
    fn load_path(path: &Path) -> Self {
        let data = match read(path) {
            Ok(data) => data,
            Err(err) => {
                error!(
                    "Failed to read sound file: {}. Error: {err} Returning default sound",
                    path.display()
                );
                DEFAULT_SOUND_DATA.into()
            }
        };

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
