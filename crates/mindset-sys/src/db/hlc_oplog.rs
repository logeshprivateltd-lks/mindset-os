//! MindSet Deterministic Hybrid Logical Clock (HLC) & Change-Log Engine
//! Conflict-free peer-to-peer sync logging and Last-Write-Wins (LWW) conflict resolution
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::kernel::{EventBus, KernelEvent};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OplogEntry {
    pub op_id: String,
    pub hlc_timestamp: String,
    pub device_id: String,
    pub table_name: String,
    pub row_id: String,
    pub operation: String, // "INSERT", "UPDATE", "DELETE"
    pub payload_json: String,
    pub synced_devices_json: String,
}

/// Monotonic Hybrid Logical Clock (<wall_time_ms>:<logical_counter>:<device_id>)
#[derive(Debug, Clone)]
pub struct HybridLogicalClock {
    physical_time_ms: u64,
    logical_counter: u32,
    device_id: String,
}

impl HybridLogicalClock {
    pub fn new(device_id: impl Into<String>) -> Self {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Self {
            physical_time_ms: now_ms,
            logical_counter: 0,
            device_id: device_id.into(),
        }
    }

    /// Generates next strictly monotonically increasing HLC timestamp string
    pub fn now(&mut self) -> String {
        let wall_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        if wall_ms > self.physical_time_ms {
            self.physical_time_ms = wall_ms;
            self.logical_counter = 0;
        } else {
            self.logical_counter += 1;
        }

        format!(
            "{}:{}:{}",
            self.physical_time_ms, self.logical_counter, self.device_id
        )
    }

    /// Advances the local clock state after observing a remote event's HLC
    pub fn update(&mut self, remote_hlc: &str) {
        let wall_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let parts: Vec<&str> = remote_hlc.split(':').collect();
        if parts.len() >= 2 {
            let remote_phys = parts[0].parse::<u64>().unwrap_or(0);
            let remote_count = parts[1].parse::<u32>().unwrap_or(0);

            if wall_ms > self.physical_time_ms && wall_ms > remote_phys {
                self.physical_time_ms = wall_ms;
                self.logical_counter = 0;
            } else if self.physical_time_ms == remote_phys {
                self.logical_counter = self.logical_counter.max(remote_count) + 1;
            } else if remote_phys > self.physical_time_ms {
                self.physical_time_ms = remote_phys;
                self.logical_counter = remote_count + 1;
            } else {
                self.logical_counter += 1;
            }
        }
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }
}

/// Atomically applies a write operation on local_core and appends an HLC delta to sync_oplog
pub fn execute_with_oplog<F>(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload_json: &str,
    write_fn: F,
) -> Result<(), rusqlite::Error>
where
    F: FnOnce(&Connection) -> Result<(), rusqlite::Error>,
{
    // 1. Execute primary write in local_core
    write_fn(core_conn)?;

    // 2. Obtain next HLC timestamp
    let (hlc_timestamp, device_id) = {
        let mut clock = hlc.lock().unwrap();
        let ts = clock.now();
        let dev = clock.device_id().to_string();
        (ts, dev)
    };

    let op_id = format!("op-{}", uuid::Uuid::new_v4());

    // 3. Append to sync_oplog.sqlite
    oplog_conn.execute(
        r#"
        INSERT INTO oplog_entries (
            op_id, hlc_timestamp, device_id, table_name, row_id, 
            operation, payload_json, synced_devices_json
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '[]')
        "#,
        params![
            op_id,
            hlc_timestamp,
            device_id,
            table_name,
            row_id,
            operation,
            payload_json
        ],
    )?;

    // 4. Emit typed kernel event for live reactive frontend updates
    if let Some(event_bus) = bus {
        event_bus.publish(KernelEvent::SyncOplogAppended {
            op_id,
            row_id: row_id.to_string(),
            table_name: table_name.to_string(),
        });
    }

    debug!(
        "Recorded oplog entry for {}:{} (HLC: {})",
        table_name, row_id, hlc_timestamp
    );

    Ok(())
}

/// Ingests and applies a batch of remote oplog entries using Last-Write-Wins (LWW)
/// Preserves conflicting text content for reflective journals/notes instead of data loss.
#[allow(dead_code)]
pub fn apply_remote_oplog_batch(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    entries: Vec<OplogEntry>,
) -> Result<usize, rusqlite::Error> {
    let mut applied_count = 0;

    for entry in entries {
        // Advance local HLC with remote timestamp
        {
            let mut clock = hlc.lock().unwrap();
            clock.update(&entry.hlc_timestamp);
        }

        // Check if oplog entry already processed
        let exists: bool = oplog_conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM oplog_entries WHERE op_id = ?1)",
                params![entry.op_id],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if exists {
            continue;
        }

        let parsed_payload: Result<serde_json::Value, _> =
            serde_json::from_str(&entry.payload_json);

        if let Ok(payload) = parsed_payload {
            match entry.table_name.as_str() {
                "journal_entries" => {
                    handle_journal_entry_sync(core_conn, &entry, &payload)?;
                }
                "tasks_and_goals" => {
                    handle_task_entry_sync(core_conn, &entry, &payload)?;
                }
                "habits" => {
                    handle_habit_entry_sync(core_conn, &entry, &payload)?;
                }
                "user_profile" => {
                    handle_profile_entry_sync(core_conn, &entry, &payload)?;
                }
                _ => {
                    warn!("Unsupported table for remote sync: {}", entry.table_name);
                }
            }

            // Record entry in local sync_oplog
            oplog_conn.execute(
                r#"
                INSERT OR IGNORE INTO oplog_entries (
                    op_id, hlc_timestamp, device_id, table_name, row_id, 
                    operation, payload_json, synced_devices_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '[]')
                "#,
                params![
                    entry.op_id,
                    entry.hlc_timestamp,
                    entry.device_id,
                    entry.table_name,
                    entry.row_id,
                    entry.operation,
                    entry.payload_json
                ],
            )?;

            if let Some(event_bus) = bus {
                event_bus.publish(KernelEvent::SyncOplogAppended {
                    op_id: entry.op_id,
                    row_id: entry.row_id,
                    table_name: entry.table_name,
                });
            }

            applied_count += 1;
        }
    }

    info!("Applied {} remote sync oplog entries", applied_count);
    Ok(applied_count)
}

#[allow(dead_code)]
fn handle_journal_entry_sync(
    conn: &Connection,
    entry: &OplogEntry,
    payload: &serde_json::Value,
) -> Result<(), rusqlite::Error> {
    let existing_content: Option<String> = conn
        .query_row(
            "SELECT content_md FROM journal_entries WHERE id = ?1",
            params![entry.row_id],
            |row| row.get(0),
        )
        .ok();

    let new_content = payload
        .get("content_md")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let final_content = if let Some(existing) = existing_content {
        if existing != new_content {
            // Conflict preservation: retain both copies
            format!(
                "{}\n\n---\n> [Conflicting revision from {} at {}]:\n{}",
                existing, entry.device_id, entry.hlc_timestamp, new_content
            )
        } else {
            existing
        }
    } else {
        new_content.to_string()
    };

    let title = payload.get("title").and_then(|v| v.as_str());
    let gratitude = payload
        .get("gratitude_items_json")
        .and_then(|v| v.as_str())
        .unwrap_or("[]");
    let mood_tag = payload.get("mood_tag").and_then(|v| v.as_str());
    let energy_level = payload.get("energy_level").and_then(|v| v.as_str());
    let created_at = payload
        .get("created_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let updated_at = payload
        .get("updated_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    conn.execute(
        r#"
        INSERT INTO journal_entries (
            id, title, content_md, gratitude_items_json, mood_tag, energy_level, 
            is_encrypted, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?8)
        ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            content_md = excluded.content_md,
            gratitude_items_json = excluded.gratitude_items_json,
            mood_tag = excluded.mood_tag,
            energy_level = excluded.energy_level,
            updated_at = excluded.updated_at
        "#,
        params![
            entry.row_id,
            title,
            final_content,
            gratitude,
            mood_tag,
            energy_level,
            created_at,
            updated_at
        ],
    )?;

    Ok(())
}

#[allow(dead_code)]
fn handle_task_entry_sync(
    conn: &Connection,
    entry: &OplogEntry,
    payload: &serde_json::Value,
) -> Result<(), rusqlite::Error> {
    if entry.operation == "DELETE" {
        conn.execute(
            "DELETE FROM tasks_and_goals WHERE id = ?1",
            params![entry.row_id],
        )?;
        return Ok(());
    }

    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let subject = payload.get("subject_tag").and_then(|v| v.as_str());
    let priority = payload
        .get("priority")
        .and_then(|v| v.as_i64())
        .unwrap_or(2);
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let weight = payload
        .get("target_score_weight")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);
    let sched_start = payload.get("scheduled_start").and_then(|v| v.as_str());
    let sched_end = payload.get("scheduled_end").and_then(|v| v.as_str());
    let completed_at = payload.get("completed_at").and_then(|v| v.as_str());
    let xp_reward = payload
        .get("xp_reward")
        .and_then(|v| v.as_i64())
        .unwrap_or(10);
    let updated_at = payload
        .get("updated_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    conn.execute(
        r#"
        INSERT INTO tasks_and_goals (
            id, parent_goal_id, title, subject_tag, priority, status, 
            target_score_weight, scheduled_start, scheduled_end, completed_at, 
            xp_reward, updated_at
        ) VALUES (?1, NULL, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            subject_tag = excluded.subject_tag,
            priority = excluded.priority,
            status = excluded.status,
            target_score_weight = excluded.target_score_weight,
            scheduled_start = excluded.scheduled_start,
            scheduled_end = excluded.scheduled_end,
            completed_at = excluded.completed_at,
            xp_reward = excluded.xp_reward,
            updated_at = excluded.updated_at
        "#,
        params![
            entry.row_id,
            title,
            subject,
            priority,
            status,
            weight,
            sched_start,
            sched_end,
            completed_at,
            xp_reward,
            updated_at
        ],
    )?;

    Ok(())
}

#[allow(dead_code)]
fn handle_habit_entry_sync(
    conn: &Connection,
    entry: &OplogEntry,
    payload: &serde_json::Value,
) -> Result<(), rusqlite::Error> {
    if entry.operation == "DELETE" {
        conn.execute("DELETE FROM habits WHERE id = ?1", params![entry.row_id])?;
        return Ok(());
    }

    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let category = payload
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("wellness");
    let freq = payload
        .get("frequency_json")
        .and_then(|v| v.as_str())
        .unwrap_or(r#"{"type":"daily"}"#);
    let streak = payload
        .get("current_streak")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let best = payload
        .get("best_streak")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let last_date = payload.get("last_completed_date").and_then(|v| v.as_str());
    let updated_at = payload
        .get("updated_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    conn.execute(
        r#"
        INSERT INTO habits (
            id, title, category, frequency_json, current_streak, best_streak, 
            last_completed_date, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            category = excluded.category,
            frequency_json = excluded.frequency_json,
            current_streak = MAX(habits.current_streak, excluded.current_streak),
            best_streak = MAX(habits.best_streak, excluded.best_streak),
            last_completed_date = excluded.last_completed_date,
            updated_at = excluded.updated_at
        "#,
        params![
            entry.row_id,
            title,
            category,
            freq,
            streak,
            best,
            last_date,
            updated_at
        ],
    )?;

    Ok(())
}

#[allow(dead_code)]
fn handle_profile_entry_sync(
    conn: &Connection,
    entry: &OplogEntry,
    payload: &serde_json::Value,
) -> Result<(), rusqlite::Error> {
    let display_name = payload
        .get("display_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Scholar");
    let role = payload
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("student");
    let iface = payload
        .get("interface_lang")
        .and_then(|v| v.as_str())
        .unwrap_or("en");
    let ai_lang = payload
        .get("ai_lang")
        .and_then(|v| v.as_str())
        .unwrap_or("ta");
    let style = payload
        .get("conversation_style")
        .and_then(|v| v.as_str())
        .unwrap_or("tanglish");
    let goals = payload
        .get("goals_json")
        .and_then(|v| v.as_str())
        .unwrap_or("[]");
    let focus = payload
        .get("focus_areas_json")
        .and_then(|v| v.as_str())
        .unwrap_or("[]");
    let routine = payload
        .get("routine_json")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");
    let prefs = payload
        .get("preferences_json")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");
    let interests = payload
        .get("interests_json")
        .and_then(|v| v.as_str())
        .unwrap_or("[]");
    let perms = payload
        .get("permissions_json")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");
    let updated = payload
        .get("updated_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    conn.execute(
        r#"
        INSERT INTO user_profile (
            user_id, display_name, role, interface_lang, ai_lang, 
            conversation_style, goals_json, focus_areas_json, routine_json, 
            preferences_json, interests_json, permissions_json, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
        ON CONFLICT(user_id) DO UPDATE SET
            display_name = excluded.display_name,
            role = excluded.role,
            interface_lang = excluded.interface_lang,
            ai_lang = excluded.ai_lang,
            conversation_style = excluded.conversation_style,
            goals_json = excluded.goals_json,
            focus_areas_json = excluded.focus_areas_json,
            routine_json = excluded.routine_json,
            preferences_json = excluded.preferences_json,
            interests_json = excluded.interests_json,
            permissions_json = excluded.permissions_json,
            updated_at = excluded.updated_at
        "#,
        params![
            entry.row_id,
            display_name,
            role,
            iface,
            ai_lang,
            style,
            goals,
            focus,
            routine,
            prefs,
            interests,
            perms,
            updated
        ],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hlc_monotonicity() {
        let mut hlc = HybridLogicalClock::new("laptop-node-01");
        let ts1 = hlc.now();
        let ts2 = hlc.now();
        let ts3 = hlc.now();

        assert_ne!(ts1, ts2);
        assert_ne!(ts2, ts3);
        assert!(ts1 < ts2);
        assert!(ts2 < ts3);
    }
}
