use std::fs::File;
use std::io::BufReader;
use std::thread;
use rodio::{Decoder, Sink};

use crate::scaffolding::Control;
mod audio_playback;

pub(crate) fn audio_command(_message: Control, sink: &Sink) {
    println!("Current thread: {:?}", thread::current().name());

    match _message {
        Control::Play => play(sink),
        Control::Pause => pause(sink),
        Control::Forward => {}
        Control::Backward => {}
        Control::GoBack => {}
    }
}

fn play(sink: &Sink) {
    let path = "file_example_OOG_2MG.ogg";
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);
}

fn pause(sink: &Sink) {
    sink.pause();
}
