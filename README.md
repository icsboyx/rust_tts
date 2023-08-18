# Rust Text-to-Speech Application

This is a simple Rust application that converts text into speech using Google Translate's Text-to-Speech service and plays the generated audio through the default audio output device on your system. The application utilizes the `rodio` crate for audio playback and communicates with the TTS service using HTTP requests.

## Features

- Convert text to speech using Google Translate's TTS service.
- Play the generated audio through the default audio output device.
- Support for both direct text input and file-based TTS playback.
- Graceful termination using Ctrl+C signal handler.

## Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- Dependencies: `rodio`, `cpal`, `ctrlc`, `base64`, `urlencoding`, `ureq` (specified in `Cargo.toml`)

## Usage

1. Clone this repository:

   ```sh
   git https://github.com/icsboyx/rust_tts.git
   cd rust-tts-app

2. Build and run the application:
    ```sh
    cargo build; cargo run

3. Follow the on-screen instructions to input text for text-to-speech conversion. Press Ctrl+C to terminate the application.

## Configuration
The application uses Google Translate's TTS service for audio generation. If you encounter issues or want to customize the service, refer to the tts.rs module.

    ```rust
    let tts_lang = "it"; // default language
    let tts_slow = "false"; // default fast
    let tts_tld = "it"; // default top domain (TLD) 


### Contribution
Contributions are welcome! If you find a bug or want to add new features, feel free to create a pull request.

## Special Thanks

This project was inspired by the Twitch channel of [Prof. Andrea Pollini](https://www.twitch.tv/profandreapollini) and the supportive Twitch community. Thanks to their encouragement and feedback!



## License

This project is licensed under the MIT License - see the [LICENSE](https://www.mit.edu/~amini/LICENSE.md) for details.