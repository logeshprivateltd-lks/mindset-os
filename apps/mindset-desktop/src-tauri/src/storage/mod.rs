//! MindSet OS Runtime Data Directory & Vault Initializer
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Could not determine user home directory")]
    HomeNotFound,
    #[error("Failed to create directory {0}: {1}")]
    DirectoryCreationFailed(String, std::io::Error),
    #[error("File I/O failure: {0}")]
    Io(#[from] std::io::Error),
}

/// Standardized directory layout for MindSet OS runtime data
#[derive(Debug, Clone)]
pub struct MindSetPaths {
    pub base_dir: PathBuf,
    pub db_dir: PathBuf,
    pub vault_laptop_dir: PathBuf,
    pub vault_phone_dir: PathBuf,
    pub vault_textbooks_dir: PathBuf,
    pub certs_dir: PathBuf,
    pub local_core_db: PathBuf,
    pub sync_oplog_db: PathBuf,
}

impl MindSetPaths {
    /// Resolve default ~/.mindset_data paths safely across Linux & Windows
    pub fn resolve() -> Result<Self, StorageError> {
        let home = dirs::home_dir().ok_or(StorageError::HomeNotFound)?;
        let base_dir = home.join(".mindset_data");

        let db_dir = base_dir.join("db");
        let vault_dir = base_dir.join("vault");
        let vault_laptop_dir = vault_dir.join("my-laptop");
        let vault_phone_dir = vault_dir.join("my-phone");
        let vault_textbooks_dir = vault_dir.join("textbooks");
        let certs_dir = base_dir.join("certs");
        let local_core_db = db_dir.join("local_core.sqlite");
        let sync_oplog_db = db_dir.join("sync_oplog.sqlite");

        Ok(Self {
            base_dir,
            db_dir,
            vault_laptop_dir,
            vault_phone_dir,
            vault_textbooks_dir,
            certs_dir,
            local_core_db,
            sync_oplog_db,
        })
    }

    /// Recursively create all required directories with secure access permissions
    pub fn initialize_filesystem(&self) -> Result<(), StorageError> {
        let dirs_to_create: [&Path; 6] = [
            &self.base_dir,
            &self.db_dir,
            &self.vault_laptop_dir,
            &self.vault_phone_dir,
            &self.vault_textbooks_dir,
            &self.certs_dir,
        ];

        for dir in dirs_to_create {
            if !dir.exists() {
                fs::create_dir_all(dir).map_err(|e| {
                    StorageError::DirectoryCreationFailed(dir.display().to_string(), e)
                })?;
                info!("Initialized storage directory: {}", dir.display());

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(dir)?.permissions();
                    perms.set_mode(0o700); // Strict user-only permissions
                    fs::set_permissions(dir, perms)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mindset_paths_resolution() {
        let paths = MindSetPaths::resolve().expect("Failed to resolve MindSet data paths");
        assert!(paths.base_dir.ends_with(".mindset_data"));
        assert!(paths.db_dir.ends_with("db"));
        assert!(paths.vault_laptop_dir.ends_with("my-laptop"));
        assert!(paths.vault_phone_dir.ends_with("my-phone"));
        assert!(paths.vault_textbooks_dir.ends_with("textbooks"));
        assert!(paths.certs_dir.ends_with("certs"));
    }

    #[test]
    fn test_filesystem_initialization() {
        let paths = MindSetPaths::resolve().expect("Failed to resolve MindSet data paths");
        paths.initialize_filesystem().expect("Filesystem initialization must succeed");
        assert!(paths.base_dir.exists());
        assert!(paths.db_dir.exists());
        assert!(paths.vault_laptop_dir.exists());
        assert!(paths.vault_phone_dir.exists());
        assert!(paths.vault_textbooks_dir.exists());
        assert!(paths.certs_dir.exists());
    }
}
