// #![cfg(not_android)]

use std::{
    fmt::{Debug, Formatter},
    fs::read,
    io::Cursor,
    path::{Path, PathBuf},
};

use log::error;
use refs::manage::ResourceLoader;
// use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

pub struct Sound {
    path: PathBuf,
    // stream: OutputStream,
    data: Vec<u8>,
}

impl Sound {
    pub fn play(&mut self) {
        // let cursor = Cursor::new(self.data.clone());
        // let input = Decoder::new(cursor).unwrap();
        //
        // let stream = OutputStreamBuilder::open_default_stream()
        //     .expect("rodio::OutputStreamBuilder::open_default_stream()");
        // let sink = Sink::connect_new(stream.mixer());
        //
        // self.stream = stream;
        //
        // sink.set_volume(0.1);
        // sink.append(input);
        // sink.detach();
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
        // let stream =
        //     OutputStreamBuilder::open_default_stream().expect("
        // OutputStreamBuilder::open_default_stream");

        Self {
            path: name.to_string().into(),
            // stream,
            data: data.into(),
        }
    }
}

impl Debug for Sound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}
