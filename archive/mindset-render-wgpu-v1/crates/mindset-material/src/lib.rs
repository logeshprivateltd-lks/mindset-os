//! # mindset-material
//!
//! Multi-pass Dual-Kawase frosted liquid glass blur and offscreen target manager
//! for MindSet OS neuroaesthetic surface composition.

pub mod blur;
pub mod target;

pub use blur::{BlurUniforms, DualKawasePipeline};
pub use target::MaterialTargetManager;
