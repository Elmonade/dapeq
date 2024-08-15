use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, Sink};

use crate::scaffolding::Control;

pub(crate) fn audio_command(_message: Control, sink: &Sink) {
    match _message {
        Control::Play => play(sink),
        Control::Pause => pause(sink),
        Control::Forward => {}
        Control::Backward => {}
        Control::GoBack => {}
    }
}

fn play(sink: &Sink) {
    let path = "/home/jello/Downloads/sample-15s.mp3";
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
}

fn pause(sink: &Sink) {
    sink.pause();
}
