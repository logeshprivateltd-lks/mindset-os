//! MindSet Connect Protocol — Shared Peer-to-Peer Sync & Cryptographic Transport
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
}

/// Device classification within the MindSet Ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Mobile,
}

/// Granular permissions granted during QR-based peer pairing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePermissions {
    pub can_sync_tasks: bool,
    pub can_sync_vault: bool,
    pub can_receive_sensor_feed: bool,
    pub can_trigger_focus_mode: bool,
}

impl Default for DevicePermissions {
    fn default() -> Self {
        Self {
            can_sync_tasks: true,
            can_sync_vault: true,
            can_receive_sensor_feed: true,
            can_trigger_focus_mode: true,
        }
    }
}

/// QR Code payload rendered on Desktop to bootstrap peer pairing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrPairingPayload {
    pub version: u8,
    pub session_id: String,
    pub host_device_id: String,
    pub host_device_name: String,
    pub public_key_ed25519: String,
    pub local_ip_addresses: Vec<String>,
    pub port: u16,
    pub expires_at: DateTime<Utc>,
}

/// Handshake initiation sent by Mobile upon scanning Desktop QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHandshakeRequest {
    pub session_id: String,
    pub client_device_id: String,
    pub client_device_name: String,
    pub client_device_type: DeviceType,
    pub public_key_ed25519: String,
    pub timestamp_utc: DateTime<Utc>,
    pub signature_ed25519: String,
}

/// Handshake acknowledgment and certificate exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHandshakeResponse {
    pub session_id: String,
    pub accepted: bool,
    pub permissions: DevicePermissions,
    pub rejection_reason: Option<String>,
    pub timestamp_utc: DateTime<Utc>,
    pub signature_ed25519: String,
}

/// Row-level mutation operation in sync oplog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OplogAction {
    Insert,
    Update,
    Delete,
}

/// Discrete cryptographic synchronization operation with Hybrid Logical Clock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOplogItem {
    pub op_id: String,
    pub hlc_timestamp: String, // <epoch_ms>:<counter>:<device_id>
    pub device_id: String,
    pub table_name: String,
    pub row_id: String,
    pub operation: OplogAction,
    pub payload_json: String,
    pub signature_ed25519: String,
}

/// Peer-to-peer sync request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchRequest {
    pub sender_device_id: String,
    pub last_known_hlc: Option<String>,
    pub entries: Vec<SyncOplogItem>,
}

/// Peer-to-peer sync response confirmation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchResponse {
    pub acknowledged_op_ids: Vec<String>,
    pub latest_hlc: String,
    pub success: bool,
}

