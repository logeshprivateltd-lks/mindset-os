//! MindSet Connect Protocol — Shared Peer-to-Peer Sync & Cryptographic Transport
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod pairing;
pub mod sync_engine;
pub mod remote_fs;
pub mod continuity;
pub mod hardware_bridge;
pub mod security_audit;

pub use pairing::*;
pub use sync_engine::*;
pub use remote_fs::*;
pub use continuity::*;
pub use hardware_bridge::*;
pub use security_audit::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Serialization failure: {0}")]
    Serialization(#[from] serde_json::Error),
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
