//! Pure-Rust Low-RAM Lo-Fi Audio Player & Ambient Rain Generator
//!
//! Streams `assets/audio/lofi.ogg` via BufReader (<2 MB RAM).
//! If no audio file is present, generates procedural soft rainfall brown noise with 0 external assets.

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

pub struct ProceduralRainSource {
    sample_rate: u32,
    state: u64,
    filter_val: f32,
}

impl ProceduralRainSource {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            state: 0x853c49e6748fea9b,
            filter_val: 0.0,
        }
    }
}

impl Default for ProceduralRainSource {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ProceduralRainSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Fast 64-bit XorShift PRNG for zero-dependency noise
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;

        let white = ((self.state as f64 / u64::MAX as f64) as f32) * 2.0 - 1.0;
        // 1-pole low-pass filter (brown noise rainfall emulation)
        self.filter_val = (self.filter_val * 0.94) + (white * 0.05);
        Some(self.filter_val)
    }
}

impl Source for ProceduralRainSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    pub is_playing: bool,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        Self {
            _stream: stream,
            stream_handle: handle,
            sink: None,
            is_playing: false,
        }
    }

    pub fn toggle(&mut self) -> bool {
        if self.is_playing {
            self.stop();
            false
        } else {
            self.play();
            self.is_playing
        }
    }

    pub fn play(&mut self) {
        if let Some(handle) = &self.stream_handle {
            if let Ok(sink) = Sink::try_new(handle) {
                let candidates = [
                    Path::new("assets/audio/lofi.ogg"),
                    Path::new("bins/mindset-os/assets/audio/lofi.ogg"),
                ];

                for path in candidates {
                    if path.exists() {
                        if let Ok(file) = File::open(path) {
                            let reader = BufReader::new(file);
                            if let Ok(source) = Decoder::new(reader) {
                                sink.append(source.repeat_infinite());
                                sink.set_volume(0.35);
                                self.sink = Some(sink);
                                self.is_playing = true;
                                return;
                            }
                        }
                    }
                }

                // Fallback: procedural soothing rain & brown noise generator
                let rain = ProceduralRainSource::new();
                sink.append(rain);
                sink.set_volume(0.25);
                self.sink = Some(sink);
                self.is_playing = true;
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self.is_playing = false;
    }
}
