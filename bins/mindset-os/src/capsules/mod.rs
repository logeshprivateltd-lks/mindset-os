//! Capsules Subsystem for MindSet OS
//!
//! Provides structured daily capsule data models, Maindy AI generation,
//! and automated coaching loops.

pub mod generator;

pub use generator::{
    generate_capsules_for_blueprint, CapsuleGenerator, GeneratedCapsulePlan, TrackBlueprint,
};
