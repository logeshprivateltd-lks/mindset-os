//! MindSet Connect Protocol — Shared Peer-to-Peer Sync & Cryptographic Transport
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod continuity;
pub mod frame;
pub mod hardware_bridge;
pub mod pairing;
pub mod remote_fs;
pub mod security_audit;
pub mod sync_engine;

pub use continuity::*;
pub use frame::*;
pub use hardware_bridge::*;
pub use pairing::*;
pub use remote_fs::*;
pub use security_audit::*;
pub use sync_engine::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Serialization failure: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Frame payload size ({size} bytes) exceeds maximum allowable limit ({max} bytes)")]
    FramePayloadTooLarge { size: usize, max: usize },
    #[error("Cryptographic signature verification failed")]
    InvalidSignature,
    #[error("Device authorization revoked: {0}")]
    DeviceRevoked(String),
    #[error("Invalid HLC timestamp format: {0}")]
    InvalidHlc(String),
    #[error("Protocol handshake timeout")]
    HandshakeTimeout,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
