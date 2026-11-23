//! Maindy AI Subsystem for MindSet OS
//!
//! Provides dynamic local model discovery, BYOK Cloud AI providers,
//! Zero-Failure Offline Study Helper, bilingual personas, and native TTS.

pub mod agent;
pub mod engine;
pub mod offline_helper;
pub mod persona;
pub mod prompt_builder;
pub mod provider;
pub mod tts;

pub use engine::{OllamaEngine, StreamToken};
pub use offline_helper::OfflineStudyHelper;
pub use persona::{build_system_prompt, parse_dual_text, DualTextResult, MaindyStyle};
pub use prompt_builder::build_personalized_system_prompt;
pub use provider::{HybridAiEngine, ProviderStatus};
pub use tts::TtsEngine;
