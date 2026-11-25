//! MindSet Connect — Desktop Module 3: Authenticated Remote Access (`remote_fs.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use connect_protocol::{
    RemoteBrowseRequest, RemoteBrowseResponse, RemoteDeleteConfirmationGate, RemoteFileEntry,
    RemoteFilePreviewRequest, RemoteFilePreviewResponse,
};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct RemoteFsManager {
    vault_base: PathBuf,
}

impl RemoteFsManager {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let vault_base = home.join(".mindset_data").join("vault");
        let _ = fs::create_dir_all(&vault_base);
        Self { vault_base }
    }

    /// Safely resolve root path for permitted folder
    fn resolve_vault_root(&self, target_root: &str) -> PathBuf {
        match target_root {
            "my-laptop" => self.vault_base.join("my-laptop"),
            "my-phone" => self.vault_base.join("my-phone"),
            "scanned_notes" => self.vault_base.join("scanned_notes"),
            _ => self.vault_base.clone(),
        }
    }

    /// Browse permitted vault directory
    pub fn browse_directory(&self, req: RemoteBrowseRequest) -> RemoteBrowseResponse {
        let root = self.resolve_vault_root(&req.target_vault_root);
        let rel = if req.relative_path.is_empty() || req.relative_path == "/" {
            None
        } else {
            let clean_rel = req
                .relative_path
                .trim_start_matches('/')
                .trim_start_matches('\\');
            Some(clean_rel)
        };

        let target_dir = match crate::fs::explorer::resolve_safe_path(&root, rel) {
            Ok(p) => p,
            Err(err) => {
                return RemoteBrowseResponse {
                    success: false,
                    total_count: 0,
                    current_directory: req.relative_path,
                    permissions_read_only: false,
                    entries: Vec::new(),
                    error: Some(err),
                };
            }
        };

        if !target_dir.exists() {
            let _ = fs::create_dir_all(&target_dir);
        }

        let mut entries = Vec::new();

        if let Ok(read_dir) = fs::read_dir(&target_dir) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = metadata.is_dir();
                    let is_study_pdf = file_name.to_lowercase().ends_with(".pdf");
                    let is_scanned_note = file_name.starts_with("scan_")
                        || file_name.to_lowercase().ends_with(".png");

                    let modified_at = metadata
                        .modified()
                        .ok()
                        .map(chrono::DateTime::<Utc>::from)
                        .unwrap_or_else(Utc::now);

                    entries.push(RemoteFileEntry {
                        name: file_name.clone(),
                        path: entry.path().to_string_lossy().to_string(),
                        size_bytes: metadata.len(),
                        is_dir,
                        modified_at,
                        mime_type: if is_dir {
                            "inode/directory".to_string()
                        } else if is_study_pdf {
                            "application/pdf".to_string()
                        } else {
                            "text/plain".to_string()
                        },
                        is_study_pdf,
                        is_scanned_note,
                        last_read_page: None,
                    });
                }
            }
        }

        RemoteBrowseResponse {
            success: true,
            total_count: entries.len(),
            current_directory: req.relative_path,
            permissions_read_only: false,
            entries,
            error: None,
        }
    }

    /// Read up to 64KB preview of a file without full payload overhead
    pub fn get_file_preview(&self, req: RemoteFilePreviewRequest) -> RemoteFilePreviewResponse {
        let path = Path::new(&req.file_path);
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let max_bytes = req.max_preview_bytes.clamp(1024, 65536);

        if !path.exists() {
            return RemoteFilePreviewResponse {
                success: false,
                file_name,
                size_bytes: 0,
                mime_type: String::new(),
                preview_base64_or_text: String::new(),
                is_truncated: false,
                error: Some(format!("File not found: {}", req.file_path)),
            };
        }

        match fs::metadata(path) {
            Ok(metadata) => {
                let size = metadata.len();
                let is_pdf = file_name.to_lowercase().ends_with(".pdf");
                let mime = if is_pdf {
                    "application/pdf"
                } else {
                    "text/plain"
                };

                match fs::read(path) {
                    Ok(bytes) => {
                        let slice = &bytes[..bytes.len().min(max_bytes)];
                        let preview = if is_pdf {
                            match crate::fs::document::PdfEngine::parse(&bytes) {
                                Ok(crate::fs::document::DocumentContent::Pdf {
                                    pages,
                                    metadata,
                                }) => {
                                    let first_page = pages
                                        .first()
                                        .map(|p| p.text.chars().take(512).collect::<String>())
                                        .unwrap_or_default();
                                    format!(
                                        "[PDF Document: {} pages]\n{}",
                                        metadata.page_count, first_page
                                    )
                                }
                                Ok(other) => other.extract_full_text(),
                                Err(e) => {
                                    return RemoteFilePreviewResponse {
                                        success: false,
                                        file_name,
                                        size_bytes: size,
                                        mime_type: mime.to_string(),
                                        preview_base64_or_text: String::new(),
                                        is_truncated: false,
                                        error: Some(format!("Failed to parse PDF: {}", e)),
                                    };
                                }
                            }
                        } else {
                            String::from_utf8_lossy(slice).to_string()
                        };

                        RemoteFilePreviewResponse {
                            success: true,
                            file_name,
                            size_bytes: size,
                            mime_type: mime.to_string(),
                            preview_base64_or_text: preview,
                            is_truncated: size > max_bytes as u64,
                            error: None,
                        }
                    }
                    Err(e) => RemoteFilePreviewResponse {
                        success: false,
                        file_name,
                        size_bytes: size,
                        mime_type: mime.to_string(),
                        preview_base64_or_text: String::new(),
                        is_truncated: false,
                        error: Some(format!("Failed to read file preview: {}", e)),
                    },
                }
            }
            Err(e) => RemoteFilePreviewResponse {
                success: false,
                file_name,
                size_bytes: 0,
                mime_type: String::new(),
                preview_base64_or_text: String::new(),
                is_truncated: false,
                error: Some(format!("Failed to inspect file metadata: {}", e)),
            },
        }
    }

    /// Create safety deletion confirmation gate token
    pub fn request_delete_gate(
        &self,
        file_path: &str,
        requesting_device_id: &str,
    ) -> RemoteDeleteConfirmationGate {
        RemoteDeleteConfirmationGate {
            file_path: file_path.to_string(),
            requested_by_device_id: requesting_device_id.to_string(),
            confirmation_token: format!("del-gate-{}", Uuid::new_v4().simple()),
            requires_biometric_auth: true,
        }
    }
}

impl Default for RemoteFsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_fs_empty_dir_and_missing_file_fail_cleanly() {
        let base =
            std::env::temp_dir().join(format!("mindset_remote_fs_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let _ = fs::create_dir_all(base.join("my-laptop"));
        let mgr = RemoteFsManager {
            vault_base: base.clone(),
        };

        let browse = mgr.browse_directory(RemoteBrowseRequest {
            client_device_id: "dev-1".to_string(),
            target_vault_root: "my-laptop".to_string(),
            relative_path: String::new(),
            session_token: "tok".to_string(),
        });
        assert!(browse.success);
        assert!(
            browse.entries.is_empty(),
            "Empty dir must return empty entries"
        );
        assert_eq!(browse.total_count, 0);

        let traversal = mgr.browse_directory(RemoteBrowseRequest {
            client_device_id: "dev-1".to_string(),
            target_vault_root: "my-laptop".to_string(),
            relative_path: "../../etc".to_string(),
            session_token: "tok".to_string(),
        });
        assert!(!traversal.success);
        assert!(traversal.error.is_some());

        let missing = mgr.get_file_preview(RemoteFilePreviewRequest {
            client_device_id: "dev-1".to_string(),
            file_path: base
                .join("does_not_exist.pdf")
                .to_string_lossy()
                .to_string(),
            max_preview_bytes: 4096,
        });
        assert!(!missing.success);
        assert!(missing.error.unwrap_or_default().contains("File not found"));

        let _ = fs::remove_dir_all(&base);
    }
}
