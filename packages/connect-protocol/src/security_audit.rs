//! MindSet Connect — Module 6: Security & Audit Protocol (`security_audit.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Granular Folder Access Policy for Cross-Device Sync & Remote Browsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderAccessPolicy {
    pub policy_id: String,
    pub folder_path: String,
    pub display_name: String,
    pub allowed_device_ids: Vec<String>,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_delete: bool,
    pub require_biometric_gate_for_delete: bool,
    pub restricted_file_extensions: Vec<String>,
}

impl Default for FolderAccessPolicy {
    fn default() -> Self {
        Self {
            policy_id: "default-vault-policy".to_string(),
            folder_path: "vault/shared".to_string(),
            display_name: "Shared Vault".to_string(),
            allowed_device_ids: Vec::new(),
            allow_read: true,
            allow_write: true,
            allow_delete: false,
            require_biometric_gate_for_delete: true,
            restricted_file_extensions: vec![".exe".into(), ".dll".into(), ".so".into(), ".bat".into(), ".sh".into()],
        }
    }
}

/// Severity classification for Connect security events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityAuditSeverity {
    Info,
    Warning,
    SecurityAlert,
}

/// Audit event action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityAuditEventType {
    PairingRequested,
    PairingApproved,
    PairingRevoked,
    OplogSyncCompleted,
    RemoteFileBrowsed,
    RemoteFileDownloaded,
    RemoteDeleteAttempted,
    RemoteDeleteConfirmed,
    SensitiveClipboardFiltered,
    FocusLockdownEnforced,
    EmergencyBypassGranted,
    HardwareBridgeStarted,
    HardwareBridgeStopped,
    DocumentScannedAndIngested,
}

/// Persistent audit record for all cross-device operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectAuditEntry {
    pub audit_id: String,
    pub event_type: SecurityAuditEventType,
    pub severity: SecurityAuditSeverity,
    pub actor_device_id: String,
    pub actor_device_name: String,
    pub target_resource: Option<String>,
    pub ip_address: Option<String>,
    pub description: String,
    pub timestamp_utc: DateTime<Utc>,
}
