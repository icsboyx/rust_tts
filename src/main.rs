use rodio::cpal::traits::HostTrait;
use rodio::{cpal, DeviceTrait};
use rodio::{Decoder, OutputStream};

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{io, thread};
mod tts;
use colored::*;
use ctrlc::set_handler;
use std::io::Cursor;

use crate::tts::tts;

#[allow(dead_code)]
enum AudioMessage {
    PlayFile(String),
    PlayBuffer(Vec<u8>),
    Stop,
}

fn main() {
    let audio_bytes = tts::tts("Ciao Siamo Online!".to_string());

    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    for device in devices {
        println!("{:#?}", device.name().unwrap())
    }

    // Create an audio output stream
    let (_stream, handle) = OutputStream::try_default().unwrap();

    // Create a sink to the output stream
    let sink = Arc::new(Mutex::new(rodio::Sink::try_new(&handle).unwrap()));

    // Create channels for communication
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread for audio playback
    let sink_clone = Arc::clone(&sink);

    let playback_thread = thread::spawn(move || {
        audio_playback_thread(sink_clone, rx);
    });

    tx.send(AudioMessage::PlayBuffer(audio_bytes)).unwrap();
    let _user_input_thread = thread::spawn(move || {
        loop {
            // Prompt the user for input
            println!("Enter a string for TTS:");
            // Create a mutable string to store the user input
            let mut input_string = String::new();
            // Read user input and store it in the input_string variable
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            // Trim the newline character from the input
            input_string = input_string.trim().to_string();
            tx.send(AudioMessage::PlayBuffer(tts(input_string)))
                .unwrap();
        }
    });

    // Register the CTRL+C signal handler
    set_handler(move || {
        // println!("CTRL+C signal received. Terminating...");
        println!(
            "{}",
            "CTRL+C signal received. Terminating..."
                .red()
                .bold()
                .underline()
        );
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    playback_thread.join().unwrap();
}

fn audio_playback_thread(sink: Arc<Mutex<rodio::Sink>>, rx: mpsc::Receiver<AudioMessage>) {
    while let Ok(message) = rx.recv() {
        match message {
            AudioMessage::PlayFile(filename) => {
                let file = File::open(&filename).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();

                {
                    let sink = sink.lock().unwrap();
                    sink.append(source);
                }
            }
            AudioMessage::PlayBuffer(data_buffer) => {
                let cursor = Cursor::new(data_buffer);
                let decoder = Decoder::new(cursor).unwrap();

                {
                    let sink = sink.lock().unwrap();
                    sink.append(decoder);
                }
            }
            AudioMessage::Stop => {
                let sink = sink.lock().unwrap();
                sink.stop();
            }
        }
    }
}
