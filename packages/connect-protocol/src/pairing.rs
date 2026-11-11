//! MindSet Connect — Module 1: Device Pairing & Identity Protocol (`pairing.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionMode {
    LocalWifi,
    TailscaleWireguard,
    OfflineCached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePermissions {
    pub can_sync_tasks: bool,
    pub can_sync_vault: bool,
    pub can_receive_sensor_feed: bool,
    pub can_trigger_focus_mode: bool,
    pub can_browse_remote_fs: bool,
    pub can_sync_clipboard: bool,
}

impl Default for DevicePermissions {
    fn default() -> Self {
        Self {
            can_sync_tasks: true,
            can_sync_vault: true,
            can_receive_sensor_feed: true,
            can_trigger_focus_mode: true,
            can_browse_remote_fs: true,
            can_sync_clipboard: true,
        }
    }
}

/// Payload encoded inside QR code on Desktop for pairing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrPairingPayload {
    pub version: u8,
    pub session_id: String,
    pub host_device_id: String,
    pub host_device_name: String,
    pub public_key_ed25519: String,
    pub local_ip_addresses: Vec<String>,
    pub port: u16,
    pub one_time_challenge: String,
    pub expires_at: DateTime<Utc>,
}

/// Request sent by mobile phone after scanning QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHandshakeRequest {
    pub session_id: String,
    pub client_device_id: String,
    pub client_device_name: String,
    pub client_device_type: DeviceType,
    pub public_key_ed25519: String,
    pub challenge_response: String,
    pub timestamp_utc: DateTime<Utc>,
    pub signature_ed25519: String,
}

/// Response returned to mobile phone upon pairing approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHandshakeResponse {
    pub session_id: String,
    pub accepted: bool,
    pub permissions: DevicePermissions,
    pub rejection_reason: Option<String>,
    pub host_public_key: String,
    pub timestamp_utc: DateTime<Utc>,
    pub signature_ed25519: String,
}

/// Registered trusted device in device registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub public_key_ed25519: String,
    pub ip_address: String,
    pub connection_mode: ConnectionMode,
    pub is_online: bool,
    pub permissions: DevicePermissions,
    pub paired_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub is_revoked: bool,
}

/// One-click device revocation audit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRevocationRecord {
    pub device_id: String,
    pub revoked_at: DateTime<Utc>,
    pub reason: String,
    pub revoke_remote_tokens: bool,
}
