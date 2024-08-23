use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use std::io::{BufReader, Seek};
use std::{fs::File, io::Read};

pub struct Music {
    // stream: OutputStreamHandle
}

impl Music {
    pub fn new(filename: &str) -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = std::fs::File::open(filename).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        sink.detach();
        // sink
        // sink.sleep_until_end();

        // let s = Self {
        //     stream: stream_handle
        // };

        // s.stream.play_raw(source.convert_samples()).unwrap();

        // s

        Self {}
    }

    // pub fn play(&self) {
    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     stream_handle.play_raw(self.source.convert_samples());
    // }
}
