//! System Subsystem for MindSet OS
//!
//! Includes system state, persistent scratchpad, live weather/greetings,
//! and sysinfo resource metrics.

pub mod audio;
pub mod state;
pub mod sysinfo_collector;

pub use audio::AudioPlayer;
pub use state::{OllamaStatus, RecommendedCapsule, SystemState, WeatherInfo};
pub use sysinfo_collector::{SysinfoCollector, SystemMetrics};
