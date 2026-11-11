//! MindSet Connect — Module 3: Authenticated Remote Access Protocol (`remote_fs.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFileEntry {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub is_dir: bool,
    pub modified_at: DateTime<Utc>,
    pub mime_type: String,
    pub is_study_pdf: bool,
    pub is_scanned_note: bool,
    pub last_read_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBrowseRequest {
    pub client_device_id: String,
    pub target_vault_root: String, // "my-laptop", "my-phone", "textbooks"
    pub relative_path: String,
    pub session_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBrowseResponse {
    pub success: bool,
    pub entries: Vec<RemoteFileEntry>,
    pub current_directory: String,
    pub total_count: usize,
    pub permissions_read_only: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFilePreviewRequest {
    pub client_device_id: String,
    pub file_path: String,
    pub max_preview_bytes: usize, // e.g. 64KB for instant preview without full download
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFilePreviewResponse {
    pub success: bool,
    pub file_name: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub preview_base64_or_text: String,
    pub is_truncated: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDeleteConfirmationGate {
    pub file_path: String,
    pub requested_by_device_id: String,
    pub confirmation_token: String,
    pub requires_biometric_auth: bool,
}
