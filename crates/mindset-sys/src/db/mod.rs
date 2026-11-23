//! MindSet OS Low-RAM SQLite Storage Engine
//! Manages local_core.sqlite, sync_oplog.sqlite, and HLC delta replication
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod hlc_oplog;
pub mod migrations;
pub mod repositories;

#[allow(unused_imports)]
pub use hlc_oplog::{apply_remote_oplog_batch, execute_with_oplog, HybridLogicalClock, OplogEntry};
#[allow(unused_imports)]
pub use migrations::MigrationManager;
#[allow(unused_imports)]
pub use repositories::*;

use crate::storage::MindSetPaths;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Database migration failed: {0}")]
    MigrationFailed(String),
    #[error("Lock acquisition error: {0}")]
    LockError(String),
}

pub struct DatabaseManager {
    core_conn: Arc<Mutex<Connection>>,
    oplog_conn: Arc<Mutex<Connection>>,
    hlc: Arc<Mutex<HybridLogicalClock>>,
    pub core_path: PathBuf,
    pub oplog_path: PathBuf,
}

impl DatabaseManager {
    /// Initialize both local_core and sync_oplog databases with low-RAM pragmas and migrations
    pub fn initialize(paths: &MindSetPaths) -> Result<Self, DbError> {
        let mut core = Connection::open(&paths.local_core_db)?;
        let oplog = Connection::open(&paths.sync_oplog_db)?;

        Self::apply_low_ram_pragmas(&core)?;
        Self::apply_low_ram_pragmas(&oplog)?;

        // Execute verified migrations on local_core
        MigrationManager::run_migrations(&paths.local_core_db, &mut core)?;

        // Initialize sync_oplog schema
        oplog.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS oplog_entries (
                op_id TEXT PRIMARY KEY,
                hlc_timestamp TEXT NOT NULL,
                device_id TEXT NOT NULL,
                table_name TEXT NOT NULL,
                row_id TEXT NOT NULL,
                operation TEXT CHECK(operation IN ('INSERT', 'UPDATE', 'DELETE')) NOT NULL,
                payload_json TEXT NOT NULL,
                synced_devices_json TEXT NOT NULL DEFAULT '[]'
            );

            CREATE INDEX IF NOT EXISTS idx_oplog_hlc ON oplog_entries(hlc_timestamp);
            CREATE INDEX IF NOT EXISTS idx_oplog_table_row ON oplog_entries(table_name, row_id);

            CREATE TABLE IF NOT EXISTS trusted_devices (
                device_id TEXT PRIMARY KEY,
                device_name TEXT NOT NULL,
                device_type TEXT CHECK(device_type IN ('desktop', 'mobile')) NOT NULL,
                public_key_ed25519 TEXT NOT NULL,
                permissions_json TEXT NOT NULL DEFAULT '{"can_sync": true, "can_push": true}',
                last_seen_at TEXT NOT NULL,
                is_revoked INTEGER NOT NULL DEFAULT 0
            );
            "#,
        )?;

        // Ensure default student profile exists in local_core
        Self::seed_initial_profile(&core)?;

        let hlc = Arc::new(Mutex::new(HybridLogicalClock::new("desktop-node-01")));

        info!("MindSet SQLite engines initialized with low-RAM profile (<40MB RSS)");

        Ok(Self {
            core_conn: Arc::new(Mutex::new(core)),
            oplog_conn: Arc::new(Mutex::new(oplog)),
            hlc,
            core_path: paths.local_core_db.clone(),
            oplog_path: paths.sync_oplog_db.clone(),
        })
    }

    fn apply_low_ram_pragmas(conn: &Connection) -> Result<(), DbError> {
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = MEMORY;
            PRAGMA mmap_size = 33554432;  -- 32 MB Memory-Mapped I/O cap
            PRAGMA cache_size = -8000;     -- 8 MB page cache limit
            PRAGMA foreign_keys = ON;
            PRAGMA busy_timeout = 5000;
            "#,
        )?;
        Ok(())
    }

    fn seed_initial_profile(conn: &Connection) -> Result<(), DbError> {
        let profile_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM user_profile WHERE user_id = 'default_student')",
                [],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if !profile_exists {
            let now = chrono::Utc::now().to_rfc3339();
            conn.execute(
                r#"
                INSERT INTO user_profile (
                    user_id, display_name, role, interface_lang, ai_lang, 
                    conversation_style, goals_json, focus_areas_json, routine_json, 
                    preferences_json, interests_json, permissions_json, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                "#,
                rusqlite::params![
                    "default_student",
                    "MindSet Scholar",
                    "student",
                    "en",
                    "ta",
                    "tanglish",
                    r#"["Master Core Mathematics", "Score 98% in Physics", "Daily 45-min Deep Focus"]"#,
                    r#"["Mathematics", "Quantum Mechanics", "System Architecture"]"#,
                    r#"{"wake_time":"05:30","sleep_time":"22:30","deep_work_slots":["06:00-08:00","18:00-20:00"]}"#,
                    r#"{"soundscape":"rain_forest","ambient_volume":0.4}"#,
                    r#"["Astronomy", "Rust Programming", "Cognitive Neuroscience"]"#,
                    r#"{"strict_focus_mode":true,"allow_break_override":false}"#,
                    now
                ],
            )?;

            // Insert initial default task
            conn.execute(
                r#"
                INSERT OR IGNORE INTO tasks_and_goals (
                    id, parent_goal_id, title, subject_tag, priority, status, 
                    target_score_weight, scheduled_start, scheduled_end, xp_reward, updated_at
                ) VALUES (?1, NULL, ?2, ?3, ?4, 'in_progress', 1.0, ?5, NULL, 50, ?5)
                "#,
                rusqlite::params![
                    "task_initial_orientation",
                    "Complete MindSet OS Workspace Setup & Focus Calibration",
                    "Orientation",
                    1,
                    now
                ],
            )?;

            // Seed default personalization settings for default_student
            conn.execute(
                r#"
                INSERT OR IGNORE INTO user_personalization (
                    user_id, wallpaper_type, wallpaper_path, wallpaper_dimness, wallpaper_blur_px,
                    accent_color, font_scale, glass_opacity, motion_intensity, dock_position,
                    dock_autohide, dock_style, widgets_json, updated_at
                ) VALUES (?1, 'preset', 'nebula_deep', 0.25, 0, '#8B9CFF', 1.0, 0.85, 'normal', 'bottom', 0, 'floating', '{"clock":true,"target":true,"affirmation":true}', ?2)
                "#,
                rusqlite::params!["default_student", now],
            )?;
        }

        Ok(())
    }

    pub fn checkpoint_and_optimize(&self) -> Result<(), DbError> {
        let core = self
            .core_conn
            .lock()
            .map_err(|e| DbError::LockError(e.to_string()))?;
        core.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); PRAGMA optimize;")?;

        let oplog = self
            .oplog_conn
            .lock()
            .map_err(|e| DbError::LockError(e.to_string()))?;
        oplog.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); PRAGMA optimize;")?;
        Ok(())
    }

    pub fn get_core_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.core_conn)
    }

    pub fn get_oplog_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.oplog_conn)
    }

    pub fn get_hlc(&self) -> Arc<Mutex<HybridLogicalClock>> {
        Arc::clone(&self.hlc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::EventBus;

    fn setup_test_databases() -> (DatabaseManager, std::path::PathBuf) {
        let temp_dir =
            std::env::temp_dir().join(format!("mindset_db_test_{}", uuid::Uuid::new_v4()));
        let db_dir = temp_dir.join("db");
        let core_db = db_dir.join("local_core.sqlite");
        let oplog_db = db_dir.join("sync_oplog.sqlite");

        let paths = MindSetPaths {
            base_dir: temp_dir.clone(),
            db_dir,
            vault_laptop_dir: temp_dir.join("vault/my-laptop"),
            vault_phone_dir: temp_dir.join("vault/my-phone"),
            vault_textbooks_dir: temp_dir.join("vault/textbooks"),
            certs_dir: temp_dir.join("certs"),
            local_core_db: core_db,
            sync_oplog_db: oplog_db,
        };

        paths.initialize_filesystem().expect("Init filesystem");
        let db = DatabaseManager::initialize(&paths).expect("Init db");
        (db, temp_dir)
    }

    #[test]
    fn test_initial_db_setup_and_profile_seeding() {
        let (db, _temp) = setup_test_databases();
        let core = db.get_core_connection();
        let lock = core.lock().unwrap();

        let profile = profile_repo::get_profile(&lock, "default_student")
            .expect("Fetch profile")
            .expect("Profile should exist");

        assert_eq!(profile.user_id, "default_student");
        assert_eq!(profile.display_name, "MindSet Scholar");
    }

    #[test]
    fn test_task_crud_and_completion_timeline() {
        let (db, _temp) = setup_test_databases();
        let bus = EventBus::new(16);
        let core = db.get_core_connection();
        let oplog = db.get_oplog_connection();
        let hlc = db.get_hlc();

        let task = TaskOrGoal {
            id: "task-test-01".to_string(),
            parent_goal_id: None,
            title: "Study Differential Equations".to_string(),
            subject_tag: Some("Math".to_string()),
            priority: 1,
            status: "in_progress".to_string(),
            target_score_weight: 1.0,
            scheduled_start: Some("2026-11-11T09:00:00Z".to_string()),
            scheduled_end: Some("2026-11-11T11:00:00Z".to_string()),
            completed_at: None,
            xp_reward: 35,
            updated_at: "".to_string(),
        };

        // Create task
        {
            let core_lock = core.lock().unwrap();
            let oplog_lock = oplog.lock().unwrap();
            tasks_repo::create_task(&core_lock, &oplog_lock, &hlc, Some(&bus), task)
                .expect("Create task");
        }

        // Verify task exists in core and in oplog
        {
            let oplog_lock = oplog.lock().unwrap();
            let count: i64 = oplog_lock
                .query_row(
                    "SELECT COUNT(*) FROM oplog_entries WHERE row_id = 'task-test-01'",
                    [],
                    |row| row.get(0),
                )
                .expect("Query oplog");
            assert_eq!(count, 1);
        }

        // Complete task and check timeline entry
        {
            let core_lock = core.lock().unwrap();
            let oplog_lock = oplog.lock().unwrap();
            let completed = tasks_repo::complete_task(
                &core_lock,
                &oplog_lock,
                &hlc,
                Some(&bus),
                "task-test-01",
            )
            .expect("Complete task");
            assert_eq!(completed.status, "completed");
            assert!(completed.completed_at.is_some());

            let timeline_count: i64 = core_lock
                .query_row(
                    "SELECT COUNT(*) FROM timeline_events WHERE event_type = 'task_completion'",
                    [],
                    |row| row.get(0),
                )
                .expect("Query timeline");
            assert!(timeline_count >= 1);
        }
    }

    #[test]
    fn test_vector_search_and_similarity() {
        let (db, _temp) = setup_test_databases();
        let core = db.get_core_connection();
        let lock = core.lock().unwrap();

        // 3-dimensional normalized test vectors
        let vec_a = vec![1.0, 0.0, 0.0];
        let vec_b = vec![0.9, 0.1, 0.0];
        let vec_c = vec![0.0, 1.0, 0.0];

        vector_repo::insert_vector_embedding(&lock, "doc-a", "textbook", "ch1", &vec_a).unwrap();
        vector_repo::insert_vector_embedding(&lock, "doc-b", "textbook", "ch2", &vec_b).unwrap();
        vector_repo::insert_vector_embedding(&lock, "doc-c", "textbook", "ch3", &vec_c).unwrap();

        let query = vec![1.0, 0.0, 0.0];
        let results = vector_repo::search_similar_vectors(&lock, &query, 2, None).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "doc-a");
        assert!((results[0].similarity - 1.0).abs() < 1e-4);
        assert_eq!(results[1].id, "doc-b");
    }

    #[test]
    fn test_diagnostics_integrity_check() {
        let (db, _temp) = setup_test_databases();
        let core = db.get_core_connection();
        let oplog = db.get_oplog_connection();

        let core_lock = core.lock().unwrap();
        let oplog_lock = oplog.lock().unwrap();

        let report = diagnostics_repo::run_db_integrity_check_and_repair(
            &core_lock,
            &oplog_lock,
            &db.core_path,
            &db.oplog_path,
        )
        .expect("Integrity check");

        assert_eq!(report.core_integrity, "ok");
        assert_eq!(report.oplog_integrity, "ok");
    }
}
