use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub struct AudioPlayer {
    sink: Option<Arc<Sink>>,
    stream_handle: OutputStreamHandle,
    looping: bool,       // Flag to control looping
    current_track: Option<String>,  // Track currently playing
}

impl AudioPlayer {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        AudioPlayer {
            sink: None,
            stream_handle,
            looping: false,  // Default to no looping
            current_track: None,  // No track playing initially
        }
    }

    /// Set looping behavior
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    pub fn play(&mut self, path: &str) {
        // Check if the same track is already playing
        if let Some(current_track) = &self.current_track {
            if current_track == path && self.is_playing() {
                // Do nothing if the track is already playing
                return;
            }
        }

        // Stop the current track if it's playing
        if let Some(sink) = &self.sink {
            sink.stop();  // Stop current audio
        }

        // Load the new audio file
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        // Create a new sink and configure looping if needed
        let sink = Arc::new(Sink::try_new(&self.stream_handle).unwrap());

        if self.looping {
            // If looping is enabled, play the source in a loop
            sink.append(source.repeat_infinite());
        } else {
            // Otherwise, play the source once
            sink.append(source);
        }

        // Update the current track
        self.sink = Some(sink);
        self.current_track = Some(path.to_string());
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    pub fn is_playing(&self) -> bool {
        if let Some(sink) = &self.sink {
            return !sink.is_paused();
        }
        false
    }
}
