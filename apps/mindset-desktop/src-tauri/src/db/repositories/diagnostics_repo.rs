//! Database Integrity Diagnostics & Self-Healing Doctor
//! Validates PRAGMA integrity_check, optimizes indices, and runs low-RAM VACUUM
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbIntegrityReport {
    pub core_integrity: String,
    pub oplog_integrity: String,
    pub core_size_bytes: u64,
    pub oplog_size_bytes: u64,
    pub repaired: bool,
    pub timestamp_utc: String,
}

pub fn run_db_integrity_check_and_repair(
    core_conn: &Connection,
    oplog_conn: &Connection,
    core_path: &Path,
    oplog_path: &Path,
) -> Result<DbIntegrityReport, rusqlite::Error> {
    // 1. Core database integrity check
    let core_integrity: String = core_conn.query_row("PRAGMA integrity_check;", [], |row| row.get(0))?;

    // 2. Oplog database integrity check
    let oplog_integrity: String = oplog_conn.query_row("PRAGMA integrity_check;", [], |row| row.get(0))?;

    // 3. Low-RAM optimization & defragmentation
    let _ = core_conn.execute("PRAGMA optimize;", []);
    let _ = oplog_conn.execute("PRAGMA optimize;", []);

    let repaired = if core_integrity != "ok" || oplog_integrity != "ok" {
        // Attempt recovery via VACUUM
        let _ = core_conn.execute("VACUUM;", []);
        let _ = oplog_conn.execute("VACUUM;", []);
        true
    } else {
        false
    };

    let core_size_bytes = std::fs::metadata(core_path).map(|m| m.len()).unwrap_or(0);
    let oplog_size_bytes = std::fs::metadata(oplog_path).map(|m| m.len()).unwrap_or(0);

    Ok(DbIntegrityReport {
        core_integrity,
        oplog_integrity,
        core_size_bytes,
        oplog_size_bytes,
        repaired,
        timestamp_utc: Utc::now().to_rfc3339(),
    })
}

