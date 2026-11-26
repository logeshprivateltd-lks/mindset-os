//! Permission-Gated OS Agent Subsystem for Maindy AI
//!
//! Executes structured OS tool actions (creating capsules, reading notes,
//! listing folders, opening apps, controlling system settings) strictly guarded
//! by user permission sheets and session authorization sets.

use std::fs;
use std::path::PathBuf;

pub use mindset_intelligence::agent_tools::{
    action_description, extract_agent_action, requires_permission, AgentAction, SystemControlAction,
};

pub const MAX_DOCUMENT_READ_BYTES: usize = 16 * 1024; // 16 KB safe cap

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentPermissionDecision {
    AllowOnce,
    AllowSession,
    Deny,
}

/// Resolves an authorized student vault folder path safely.
pub fn resolve_vault_path(relative_or_folder: &str) -> PathBuf {
    let raw = relative_or_folder.trim();
    let clean = raw.trim_start_matches(['/', '\\']);
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let vault_root = home.join("Study Capsules");

    if clean.is_empty() || clean == "." || clean == "Study Capsules" {
        return vault_root;
    }

    if clean == "Formula Sheets" {
        return vault_root.join("Formula Sheets");
    }

    if clean == "Documents" {
        return dirs::document_dir().unwrap_or_else(|| home.join("Documents"));
    }

    if clean == "Downloads" {
        return dirs::download_dir().unwrap_or_else(|| home.join("Downloads"));
    }

    // Preserve explicit absolute filesystem paths on Unix and Windows instead of stripping '/'
    let raw_path = std::path::Path::new(raw);
    if raw_path.is_absolute() {
        return raw_path.to_path_buf();
    }

    // Check if relative to Study Capsules
    let candidate = vault_root.join(clean);
    if candidate.exists() {
        return candidate;
    }

    // Fallback: check workspace assets or documents
    let local_path = PathBuf::from(clean);
    if local_path.exists() {
        return local_path;
    }

    candidate
}

/// Safely reads a student text or notes document up to 16 KB.
/// Returns `(content, is_truncated)`.
pub fn safe_read_document(file_path: &str) -> Result<(String, bool), String> {
    let path = resolve_vault_path(file_path);

    if !path.exists() {
        return Err(format!("Document not found at: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!(
            "Target is a directory, not a file: {}",
            path.display()
        ));
    }

    let meta = fs::metadata(&path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_len = meta.len() as usize;

    let bytes = fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let is_truncated = bytes.len() > MAX_DOCUMENT_READ_BYTES;
    let slice = if is_truncated {
        &bytes[..MAX_DOCUMENT_READ_BYTES]
    } else {
        &bytes[..]
    };

    let text = String::from_utf8_lossy(slice).to_string();
    Ok((text, is_truncated || file_len > MAX_DOCUMENT_READ_BYTES))
}

/// Safely lists filenames inside a student vault folder.
pub fn safe_list_folder(folder_path: &str) -> Result<Vec<String>, String> {
    let path = resolve_vault_path(folder_path);

    if !path.exists() {
        // Create if it's standard vault folder
        let _ = fs::create_dir_all(&path);
    }

    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }

    let mut entries = Vec::new();
    let read_dir = fs::read_dir(&path).map_err(|e| format!("Could not open directory: {}", e))?;

    for item in read_dir.flatten() {
        if let Ok(file_name) = item.file_name().into_string() {
            let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let size = item.metadata().map(|m| m.len()).unwrap_or(0);
            if is_dir {
                entries.push(format!("📁 {}/", file_name));
            } else {
                entries.push(format!("📄 {} ({:.1} KB)", file_name, size as f32 / 1024.0));
            }
        }
    }

    entries.sort();
    Ok(entries)
}

/// Checks if a given file or folder path has already been granted session permission.
pub fn is_folder_permitted(
    path: &str,
    session_allowed: &std::collections::HashSet<String>,
) -> bool {
    let clean = path.trim().replace('\\', "/").to_lowercase();

    for allowed in session_allowed {
        let allowed_clean = allowed.trim().replace('\\', "/").to_lowercase();
        if clean == allowed_clean
            || clean.starts_with(&format!("{}/", allowed_clean))
            || clean.contains(&format!("{}/", allowed_clean))
            || clean.ends_with(&allowed_clean)
        {
            return true;
        }
    }

    false
}

/// Extracts folder name from a file or directory path for session authorization
pub fn extract_folder_scope(path: &str) -> String {
    let clean = path.trim().replace('\\', "/");
    let parts: Vec<&str> = clean.split('/').filter(|s| !s.is_empty()).collect();

    if let Some(first) = parts.first() {
        first.to_string()
    } else {
        "Study Capsules".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_interlock_blocks_unauthorized_paths() {
        let session_allowed = std::collections::HashSet::new();

        // All paths must be blocked when session_allowed is empty
        assert!(!is_folder_permitted(
            "Formula Sheets/Physics_Formula_Sheet.txt",
            &session_allowed
        ));
        assert!(!is_folder_permitted("Study Capsules", &session_allowed));
        assert!(!is_folder_permitted(
            "Documents/Personal.txt",
            &session_allowed
        ));
        assert!(!is_folder_permitted("Downloads", &session_allowed));
    }

    #[test]
    fn test_session_folder_scope_authorization() {
        let mut session_allowed = std::collections::HashSet::new();
        let scope = extract_folder_scope("Formula Sheets/Physics_Formula_Sheet.txt");
        assert_eq!(scope, "Formula Sheets");

        session_allowed.insert(scope);

        // Formula Sheets should now be permitted
        assert!(is_folder_permitted(
            "Formula Sheets/Physics_Formula_Sheet.txt",
            &session_allowed
        ));
        assert!(is_folder_permitted("Formula Sheets", &session_allowed));

        // Documents and other folders must remain strictly BLOCKED
        assert!(!is_folder_permitted(
            "Documents/Personal.txt",
            &session_allowed
        ));
        assert!(!is_folder_permitted(
            "Study Capsules/Math.json",
            &session_allowed
        ));
    }

    #[test]
    fn test_safe_read_document_file_size_cap() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join(format!("mindset_test_oversized_{}.txt", std::process::id()));

        // Write 25 KB of sample data (exceeding 16 KB / MAX_DOCUMENT_READ_BYTES)
        let sample_data = "A".repeat(25 * 1024);
        fs::write(&test_file, &sample_data).expect("write oversized test file");

        let (content, is_truncated) =
            safe_read_document(&test_file.to_string_lossy()).expect("safe_read_document");
        assert_eq!(content.len(), MAX_DOCUMENT_READ_BYTES);
        assert!(is_truncated);

        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_vault_path_resolution() {
        let resolved = resolve_vault_path("Formula Sheets");
        let path_str = resolved.to_string_lossy();
        assert!(path_str.contains("Formula Sheets"));

        let leading_slash_vault = resolve_vault_path("/Formula Sheets");
        assert_eq!(leading_slash_vault, resolved);

        #[cfg(unix)]
        {
            let abs = resolve_vault_path("/tmp/mindset_abs_probe.txt");
            assert_eq!(abs, PathBuf::from("/tmp/mindset_abs_probe.txt"));
        }
        #[cfg(windows)]
        {
            let abs = resolve_vault_path(r"C:\Temp\mindset_abs_probe.txt");
            assert_eq!(abs, PathBuf::from(r"C:\Temp\mindset_abs_probe.txt"));
        }
    }
}
