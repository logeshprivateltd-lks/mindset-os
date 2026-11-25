//! Headless Test Harness for MindSet OS Integration & E2E Testing
//!
//! Provides isolated in-memory SQLite databases, tempfile scratch vaults,
//! and headless synchronous message dispatching against `MindSetApp`.
//! Zero GPU/window allocations, zero external server dependencies.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use tempfile::TempDir;

use mindset_core::{ActiveOverlay, WindowId};
use mindset_os::{Message, MindSetApp};
use mindset_sys::auth::CredentialsManager;

static TEST_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Global lock ensuring tests exercising auth credentials do not collide across threads
pub static AUTH_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Headless test fixture encapsulating an isolated MindSetApp instance
pub struct HeadlessHarness {
    pub app: MindSetApp,
    pub temp_dir: TempDir,
    pub vault_dir: PathBuf,
    pub db_path: PathBuf,
}

impl Default for HeadlessHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlessHarness {
    /// Creates a pristine, fully isolated test environment with fresh tempdir and seeded DB
    pub fn new() -> Self {
        let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = tempfile::Builder::new()
            .prefix(&format!("mindset_test_{test_id}_"))
            .tempdir()
            .expect("Failed to create temporary directory for test harness");

        let vault_dir = temp_dir.path().join("vault");
        std::fs::create_dir_all(&vault_dir).expect("Failed to create isolated vault directory");

        let db_path = temp_dir.path().join("test_mindset_os.db");

        // Initialize production SQLite schema & migrations in the isolated database file
        let storage = mindset_os::storage::StorageManager::with_path(db_path.clone());
        let conn = storage
            .open_connection()
            .expect("Failed to open isolated test SQLite database");

        // Seed default credentials for "scholar" and "Mapla"
        let _ = CredentialsManager::ensure_default_credentials(&conn, "scholar");
        let _ = CredentialsManager::ensure_default_credentials(&conn, "Mapla");

        // Seed default test user profile using production StorageManager
        let mut profile = storage.load_profile();
        profile.name = "Mapla".to_string();
        profile.career_track = "Class12BoardsTN600".to_string();
        profile.target_score_value = 590;
        profile.target_score_max = 600;
        profile.exam_date = "2027-03-01".to_string();
        profile.subjects = vec![
            "Physics".to_string(),
            "Chemistry".to_string(),
            "Mathematics".to_string(),
        ];
        profile.weak_topics = vec![
            "Gauss Law & Flux".to_string(),
            "Electrochemistry Nernst Eq".to_string(),
        ];
        profile.weekday_study_hours = 4;
        profile.peak_energy_window = "06:00 - 10:00".to_string();
        storage
            .save_profile(&profile)
            .expect("Failed to seed default test user profile");

        let app = MindSetApp::with_storage_path(db_path.clone());

        Self {
            app,
            temp_dir,
            vault_dir,
            db_path,
        }
    }

    /// Synchronously dispatches a single message to the Elm architecture loop
    pub fn dispatch(&mut self, message: Message) {
        let _ = self.app.update(message);
    }

    /// Dispatches a slice of messages in sequence
    pub fn dispatch_batch(&mut self, messages: &[Message]) {
        for msg in messages {
            let _ = self.app.update(msg.clone());
        }
    }

    /// Returns the active overlay
    pub fn active_overlay(&self) -> ActiveOverlay {
        self.app.core.wm.active_overlay
    }

    /// Checks if a window is currently open
    pub fn is_window_open(&self, window_id: WindowId) -> bool {
        self.app.core.wm.is_open(window_id)
    }

    /// Opens an in-memory SQLite connection for testing db queries directly
    pub fn open_test_conn(&self) -> rusqlite::Connection {
        rusqlite::Connection::open(&self.db_path)
            .expect("Failed to open connection to test sqlite database")
    }

    /// Access the temporary vault path
    pub fn vault_path(&self) -> &Path {
        &self.vault_dir
    }

    /// Resets the auth_credentials table for user 'scholar' and lock_screen UI state
    pub fn reset_auth(&mut self) {
        if let Ok(conn) = self.app.storage.open_connection() {
            let _ = conn.execute(
                "UPDATE auth_credentials SET failed_attempts = 0, locked_until = NULL WHERE user_id = 'scholar'",
                [],
            );
        }
        self.app.lock_screen = Default::default();
    }
}
