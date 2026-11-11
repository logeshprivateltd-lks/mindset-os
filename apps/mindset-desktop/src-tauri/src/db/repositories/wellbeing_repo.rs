//! Wellbeing Repository: Reflective Journal, Mood Check-ins, and Focus Sessions
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::hlc_oplog::{execute_with_oplog, HybridLogicalClock};
use crate::kernel::{EventBus, KernelEvent};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub title: Option<String>,
    pub content_md: String,
    pub gratitude_items_json: String,
    pub mood_tag: Option<String>,
    pub energy_level: Option<String>,
    pub is_encrypted: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodCheckin {
    pub id: String,
    pub mood: String,
    pub energy_tier: String,
    pub context_note: Option<String>,
    pub active_activity: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: String,
    pub profile_mode: String,
    pub planned_minutes: i64,
    pub actual_minutes: i64,
    pub task_id: Option<String>,
    pub interruption_count: i64,
    pub completed: i64,
    pub started_at: String,
    pub ended_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Journal Entries
// ---------------------------------------------------------------------------

pub fn list_journals(conn: &Connection, limit: usize) -> Result<Vec<JournalEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, title, content_md, gratitude_items_json, mood_tag, energy_level, is_encrypted, created_at, updated_at
        FROM journal_entries ORDER BY created_at DESC LIMIT ?1
        "#,
    )?;

    let mut rows = stmt.query(params![limit as i64])?;
    let mut entries = Vec::new();

    while let Some(row) = rows.next()? {
        entries.push(JournalEntry {
            id: row.get(0)?,
            title: row.get(1)?,
            content_md: row.get(2)?,
            gratitude_items_json: row.get(3)?,
            mood_tag: row.get(4)?,
            energy_level: row.get(5)?,
            is_encrypted: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        });
    }

    Ok(entries)
}

pub fn create_journal(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut entry: JournalEntry,
) -> Result<JournalEntry, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    entry.created_at = now.clone();
    entry.updated_at = now.clone();
    if entry.id.is_empty() {
        entry.id = format!("journal-{}", uuid::Uuid::new_v4());
    }

    let payload_json = serde_json::to_string(&entry).unwrap_or_default();
    let e_clone = entry.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "journal_entries",
        &e_clone.id,
        "INSERT",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO journal_entries (
                    id, title, content_md, gratitude_items_json, mood_tag, 
                    energy_level, is_encrypted, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                "#,
                params![
                    e_clone.id,
                    e_clone.title,
                    e_clone.content_md,
                    e_clone.gratitude_items_json,
                    e_clone.mood_tag,
                    e_clone.energy_level,
                    e_clone.is_encrypted,
                    e_clone.created_at,
                    e_clone.updated_at
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(entry)
}

// ---------------------------------------------------------------------------
// Mood Check-ins
// ---------------------------------------------------------------------------

pub fn list_moods(conn: &Connection, limit: usize) -> Result<Vec<MoodCheckin>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, mood, energy_tier, context_note, active_activity, created_at
        FROM mood_checkins ORDER BY created_at DESC LIMIT ?1
        "#,
    )?;

    let mut rows = stmt.query(params![limit as i64])?;
    let mut checkins = Vec::new();

    while let Some(row) = rows.next()? {
        checkins.push(MoodCheckin {
            id: row.get(0)?,
            mood: row.get(1)?,
            energy_tier: row.get(2)?,
            context_note: row.get(3)?,
            active_activity: row.get(4)?,
            created_at: row.get(5)?,
        });
    }

    Ok(checkins)
}

pub fn create_mood(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut checkin: MoodCheckin,
) -> Result<MoodCheckin, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    checkin.created_at = now.clone();
    if checkin.id.is_empty() {
        checkin.id = format!("mood-{}", uuid::Uuid::new_v4());
    }

    let payload_json = serde_json::to_string(&checkin).unwrap_or_default();
    let m_clone = checkin.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "mood_checkins",
        &m_clone.id,
        "INSERT",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO mood_checkins (
                    id, mood, energy_tier, context_note, active_activity, created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    m_clone.id,
                    m_clone.mood,
                    m_clone.energy_tier,
                    m_clone.context_note,
                    m_clone.active_activity,
                    m_clone.created_at
                ],
            )?;

            // Record timeline event
            let event_id = format!("timeline-{}", uuid::Uuid::new_v4());
            conn.execute(
                "INSERT INTO timeline_events (id, event_type, title, metadata_json, created_at) VALUES (?1, 'mood_logged', ?2, ?3, ?4)",
                params![
                    event_id,
                    format!("Mood Logged: {} ({})", m_clone.mood, m_clone.energy_tier),
                    payload_json,
                    now
                ],
            )?;

            Ok(())
        },
    )?;

    if let Some(event_bus) = bus {
        event_bus.publish(KernelEvent::MoodLogged {
            mood: checkin.mood.clone(),
            energy_tier: checkin.energy_tier.clone(),
        });
    }

    Ok(checkin)
}

// ---------------------------------------------------------------------------
// Focus Sessions
// ---------------------------------------------------------------------------

pub fn list_focus_sessions(conn: &Connection, limit: usize) -> Result<Vec<FocusSession>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, profile_mode, planned_minutes, actual_minutes, task_id, interruption_count, completed, started_at, ended_at
        FROM focus_sessions ORDER BY started_at DESC LIMIT ?1
        "#,
    )?;

    let mut rows = stmt.query(params![limit as i64])?;
    let mut sessions = Vec::new();

    while let Some(row) = rows.next()? {
        sessions.push(FocusSession {
            id: row.get(0)?,
            profile_mode: row.get(1)?,
            planned_minutes: row.get(2)?,
            actual_minutes: row.get(3)?,
            task_id: row.get(4)?,
            interruption_count: row.get(5)?,
            completed: row.get(6)?,
            started_at: row.get(7)?,
            ended_at: row.get(8)?,
        });
    }

    Ok(sessions)
}

pub fn start_focus_session(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut session: FocusSession,
) -> Result<FocusSession, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    session.started_at = now.clone();
    if session.id.is_empty() {
        session.id = format!("focus-{}", uuid::Uuid::new_v4());
    }

    let payload_json = serde_json::to_string(&session).unwrap_or_default();
    let s_clone = session.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "focus_sessions",
        &s_clone.id,
        "INSERT",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO focus_sessions (
                    id, profile_mode, planned_minutes, actual_minutes, task_id, 
                    interruption_count, completed, started_at, ended_at
                ) VALUES (?1, ?2, ?3, 0, ?4, 0, 0, ?5, NULL)
                "#,
                params![
                    s_clone.id,
                    s_clone.profile_mode,
                    s_clone.planned_minutes,
                    s_clone.task_id,
                    s_clone.started_at
                ],
            )?;
            Ok(())
        },
    )?;

    if let Some(event_bus) = bus {
        event_bus.publish(KernelEvent::FocusSessionStateChanged {
            session_id: session.id.clone(),
            state: "STARTED".to_string(),
            elapsed_secs: 0,
        });
    }

    Ok(session)
}

pub fn end_focus_session(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    session_id: &str,
    actual_minutes: i64,
    interruption_count: i64,
) -> Result<(), rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    let payload = serde_json::json!({
        "id": session_id,
        "actual_minutes": actual_minutes,
        "interruption_count": interruption_count,
        "completed": 1,
        "ended_at": now
    });
    let payload_str = payload.to_string();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "focus_sessions",
        session_id,
        "UPDATE",
        &payload_str,
        |conn| {
            conn.execute(
                r#"
                UPDATE focus_sessions SET
                    actual_minutes = ?1,
                    interruption_count = ?2,
                    completed = 1,
                    ended_at = ?3
                WHERE id = ?4
                "#,
                params![actual_minutes, interruption_count, now, session_id],
            )?;

            // Record timeline event
            let event_id = format!("timeline-{}", uuid::Uuid::new_v4());
            conn.execute(
                "INSERT INTO timeline_events (id, event_type, title, metadata_json, created_at) VALUES (?1, 'focus_session', ?2, ?3, ?4)",
                params![
                    event_id,
                    format!("Focus Session Finished: {} min block", actual_minutes),
                    payload_str,
                    now
                ],
            )?;

            Ok(())
        },
    )?;

    if let Some(event_bus) = bus {
        event_bus.publish(KernelEvent::FocusSessionStateChanged {
            session_id: session_id.to_string(),
            state: "COMPLETED".to_string(),
            elapsed_secs: (actual_minutes * 60) as u32,
        });
    }

    Ok(())
}
