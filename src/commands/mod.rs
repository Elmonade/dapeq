use rodio::Sink;

use crate::scaffolding::Control;

pub(crate) fn audio_command(_message: Control, sink: &Sink) {
    match _message {
        Control::PlayPause => play_pause(sink),
        Control::Forward => skip_forward(sink),
        Control::Backward => skip_backward(sink),
        Control::GoBack => {}
    }
}

fn play_pause(sink: &Sink) {
    match sink.is_paused() {
        false => sink.pause(),
        true => sink.play(),
    }
}

fn skip_forward(sink: &Sink) {
    // Assuming we will always have a queue.
    // Also, it needs to loop around.
    sink.skip_one();
}

fn skip_backward(_sink: &Sink) {
    //TODO: Play the previous song from the "cache?".
    // Set to 00:00 if >5sec, otherwise skip the current song.
}
