use std::fs::File;
use std::io::BufReader;
use std::thread;
use rodio::{Decoder, OutputStream, Sink};

pub fn play() {
    let path = "file_example_OOG_2MG.ogg";
    let _ = thread::Builder::new()
        .name("music_player".to_string())
        .spawn(move || {
            println!("Received play command.");

            // _stream must live as long as the sink
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open(path).unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            sink.append(source);

            // The sound plays in a separate thread.
            // This call will block the current thread until the sink
            // has finished playing all its queued sounds.

            // pause(&sink);

            // sink.sleep_until_end();
    });
}

pub fn pause(sink: &Sink) {
    sink.pause();
}
