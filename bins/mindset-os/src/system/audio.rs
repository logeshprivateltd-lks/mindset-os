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

pub struct ProceduralChimeSource {
    sample_rate: u32,
    sample_idx: usize,
    total_samples: usize,
    freq1: f32,
    freq2: f32,
    freq3: f32,
}

impl ProceduralChimeSource {
    pub fn new() -> Self {
        let sample_rate = 44100;
        let duration_secs = 0.65;
        Self {
            sample_rate,
            sample_idx: 0,
            total_samples: (sample_rate as f32 * duration_secs).round() as usize,
            freq1: 523.25, // C5
            freq2: 659.25, // E5
            freq3: 783.99, // G5
        }
    }
}

impl Default for ProceduralChimeSource {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ProceduralChimeSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_idx >= self.total_samples {
            return None;
        }
        let t = self.sample_idx as f32 / self.sample_rate as f32;
        self.sample_idx += 1;

        let decay = (-5.2 * t).exp();
        let s1 = (2.0 * std::f32::consts::PI * self.freq1 * t).sin() * 0.40;
        let s2 = (2.0 * std::f32::consts::PI * self.freq2 * t).sin() * 0.35;
        let s3 = (2.0 * std::f32::consts::PI * self.freq3 * t).sin() * 0.25;

        Some((s1 + s2 + s3) * decay * 0.32)
    }
}

impl Source for ProceduralChimeSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.total_samples.saturating_sub(self.sample_idx))
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.total_samples as f32 / self.sample_rate as f32,
        ))
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

    pub fn play_chime(&self) {
        if let Some(handle) = &self.stream_handle {
            let candidates = [
                Path::new("assets/audio/chime.ogg"),
                Path::new("bins/mindset-os/assets/audio/chime.ogg"),
            ];

            for path in candidates {
                if path.exists() {
                    if let Ok(file) = File::open(path) {
                        let reader = BufReader::new(file);
                        if let Ok(source) = Decoder::new(reader) {
                            if let Ok(sink) = Sink::try_new(handle) {
                                sink.set_volume(0.40);
                                sink.append(source);
                                sink.detach();
                                return;
                            }
                        }
                    }
                }
            }

            // Zero-asset procedural C-major bell chord
            if let Ok(sink) = Sink::try_new(handle) {
                sink.set_volume(0.35);
                sink.append(ProceduralChimeSource::new());
                sink.detach();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedural_chime_rounding_and_samples() {
        let mut chime = ProceduralChimeSource::new();
        assert_eq!(chime.total_samples, 28665);

        let mut count = 0;
        let mut max_abs: f32 = 0.0;
        let mut tail_samples = Vec::new();

        for s in chime.by_ref() {
            assert!(s.is_finite(), "Sample must be finite");
            assert!(s.abs() <= 1.0, "Sample must be bounded within [-1.0, 1.0]");
            if s.abs() > max_abs {
                max_abs = s.abs();
            }
            if count >= 28665 - 100 {
                tail_samples.push(s.abs());
            }
            count += 1;
        }

        assert_eq!(count, 28665);
        assert!(
            max_abs <= 0.40,
            "Peak amplitude should be within safe headroom"
        );
        assert!(chime.next().is_none());

        // Tail samples decay to near silence (< 0.02)
        for val in tail_samples {
            assert!(val < 0.02, "Tail sample {val} must decay to near silence");
        }
    }
}
