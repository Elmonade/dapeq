mod audio_playback;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub(crate) fn play(sink: &Sink) {
    let path = "file_example_OOG_2MG.ogg";
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);
}

pub(crate) fn pause(sink: &Sink) {
    sink.pause();
}

pub(crate) fn init() {
    println!("Initializing the music library");
    let path = "file_example_OOG_2MG.ogg";

    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.pause();

    // The sound plays in a separate thread.
    // This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
