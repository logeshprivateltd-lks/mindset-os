//! MindSet Connect — Module 2: Conflict-Aware Sync Protocol (`sync_engine.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.
//!
//! STRICT RULE 1: Never copy and overwrite raw local_core.sqlite files between devices.
//! Synchronise records strictly via sync_oplog.sqlite HLC change-logs and preserve
//! conflicting file/note versions so user work is never silently overwritten.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchRequest {
    pub sender_device_id: String,
    pub last_known_hlc: Option<String>,
    pub oplog_entries: Vec<SyncOplogItem>,
    pub timestamp_utc: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchResponse {
    pub acknowledged_op_ids: Vec<String>,
    pub incoming_oplog_entries: Vec<SyncOplogItem>,
    pub latest_hlc: String,
    pub success: bool,
    pub conflicts_detected: Vec<SyncConflictItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolutionChoice {
    KeepMine,
    KeepTheirs,
    KeepBoth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflictItem {
    pub conflict_id: String,
    pub resource_type: String, // "task", "habit", "journal_note", "file"
    pub resource_id: String,
    pub resource_title: String,
    pub local_hlc: String,
    pub local_updated_at: DateTime<Utc>,
    pub remote_hlc: String,
    pub remote_device_id: String,
    pub remote_updated_at: DateTime<Utc>,
    pub local_preview: String,
    pub remote_preview: String,
    pub resolution: Option<ConflictResolutionChoice>,
    pub preserved_conflict_filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectiveFolderSyncStatus {
    pub folder_name: String,
    pub local_path: String,
    pub remote_path: String,
    pub is_enabled: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub files_in_sync: usize,
    pub pending_uploads: usize,
    pub pending_downloads: usize,
    pub active_conflicts: usize,
}
