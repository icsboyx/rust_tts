use crate::AudioMessage;
use soloud::*;
use std::sync::mpsc;

pub fn send_audio_to_sink(rx: mpsc::Receiver<AudioMessage>) {
    let mut sl = Soloud::default().unwrap();
    let flags = soloud::prelude::SoloudFlag::all();
    let backend = soloud::Backend::Auto;
    let samplerate = sl.backend_samplerate();
    let buf_size = sl.backend_buffer_size();
    let channels = 1;
    sl.init_ex(flags, backend, samplerate, buf_size, channels)
        .unwrap();

    println!("{:#?}", sl.backend_samplerate());

    let mut wav = audio::Wav::default();

    while let Ok(message) = rx.recv() {
        match message {
            AudioMessage::PlayBuffer(data_buffer) => {
                wav.load_mem(&data_buffer).unwrap();
                sl.play(&wav);
            }
        };
    }
}
