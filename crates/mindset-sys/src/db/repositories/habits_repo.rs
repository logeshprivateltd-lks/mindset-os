//! Habit Tracking & Non-Punitive Streak Engine Repository
//! Preserves momentum without demoralizing full resets on missed days
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::hlc_oplog::{execute_with_oplog, HybridLogicalClock};
use crate::kernel::EventBus;
use chrono::{Datelike, Local, NaiveDate, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: String,
    pub title: String,
    pub category: String,
    pub frequency_json: String,
    pub current_streak: i64,
    pub best_streak: i64,
    pub last_completed_date: Option<String>,
    pub updated_at: String,
}

pub fn list_habits(conn: &Connection) -> Result<Vec<Habit>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, title, category, frequency_json, current_streak, best_streak, last_completed_date, updated_at
        FROM habits ORDER BY current_streak DESC, title ASC
        "#,
    )?;

    let mut rows = stmt.query([])?;
    let mut habits = Vec::new();

    while let Some(row) = rows.next()? {
        habits.push(Habit {
            id: row.get(0)?,
            title: row.get(1)?,
            category: row.get(2)?,
            frequency_json: row.get(3)?,
            current_streak: row.get(4)?,
            best_streak: row.get(5)?,
            last_completed_date: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }

    Ok(habits)
}

pub fn create_habit(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut habit: Habit,
) -> Result<Habit, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    habit.updated_at = now.clone();
    if habit.id.is_empty() {
        habit.id = format!("habit-{}", uuid::Uuid::new_v4());
    }

    let payload_json = serde_json::to_string(&habit).unwrap_or_default();
    let h_clone = habit.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "habits",
        &h_clone.id,
        "INSERT",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO habits (
                    id, title, category, frequency_json, current_streak, 
                    best_streak, last_completed_date, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    h_clone.id,
                    h_clone.title,
                    h_clone.category,
                    h_clone.frequency_json,
                    h_clone.current_streak,
                    h_clone.best_streak,
                    h_clone.last_completed_date,
                    h_clone.updated_at
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(habit)
}

pub fn update_habit(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut habit: Habit,
) -> Result<Habit, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    habit.updated_at = now.clone();

    let payload_json = serde_json::to_string(&habit).unwrap_or_default();
    let h_clone = habit.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "habits",
        &h_clone.id,
        "UPDATE",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                UPDATE habits SET
                    title = ?1,
                    category = ?2,
                    frequency_json = ?3,
                    current_streak = ?4,
                    best_streak = ?5,
                    last_completed_date = ?6,
                    updated_at = ?7
                WHERE id = ?8
                "#,
                params![
                    h_clone.title,
                    h_clone.category,
                    h_clone.frequency_json,
                    h_clone.current_streak,
                    h_clone.best_streak,
                    h_clone.last_completed_date,
                    h_clone.updated_at,
                    h_clone.id
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(habit)
}

pub fn delete_habit(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    habit_id: &str,
) -> Result<(), rusqlite::Error> {
    let payload_json = serde_json::json!({ "id": habit_id }).to_string();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "habits",
        habit_id,
        "DELETE",
        &payload_json,
        |conn| {
            conn.execute("DELETE FROM habits WHERE id = ?1", params![habit_id])?;
            Ok(())
        },
    )
}

/// Records habit completion for today with non-punitive streak recovery
pub fn record_habit_completion(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    habit_id: &str,
) -> Result<Habit, rusqlite::Error> {
    let now_utc = Utc::now().to_rfc3339();
    let today = Local::now().date_naive();
    let today_str = format!(
        "{:04}-{:02}-{:02}",
        today.year(),
        today.month(),
        today.day()
    );

    let mut habit: Habit = core_conn.query_row(
        "SELECT id, title, category, frequency_json, current_streak, best_streak, last_completed_date, updated_at FROM habits WHERE id = ?1",
        params![habit_id],
        |row| {
            Ok(Habit {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                frequency_json: row.get(3)?,
                current_streak: row.get(4)?,
                best_streak: row.get(5)?,
                last_completed_date: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )?;

    // Non-punitive streak calculation
    if let Some(ref last_date_str) = habit.last_completed_date {
        if last_date_str == &today_str {
            // Already logged today; no change to streak
            return Ok(habit);
        } else if let Ok(last_date) = NaiveDate::parse_from_str(last_date_str, "%Y-%m-%d") {
            let days_diff = (today - last_date).num_days();
            if days_diff == 1 {
                // Completed yesterday: streak increment
                habit.current_streak += 1;
            } else if days_diff > 1 {
                // Missed day: non-punitive recovery (starts back at 1 without destroying best_streak)
                habit.current_streak = 1;
            }
        } else {
            habit.current_streak += 1;
        }
    } else {
        habit.current_streak = 1;
    }

    if habit.current_streak > habit.best_streak {
        habit.best_streak = habit.current_streak;
    }

    habit.last_completed_date = Some(today_str);
    habit.updated_at = now_utc.clone();

    let payload_json = serde_json::to_string(&habit).unwrap_or_default();
    let h_clone = habit.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "habits",
        &h_clone.id,
        "UPDATE",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                UPDATE habits SET 
                    current_streak = ?1, 
                    best_streak = ?2, 
                    last_completed_date = ?3, 
                    updated_at = ?4 
                WHERE id = ?5
                "#,
                params![
                    h_clone.current_streak,
                    h_clone.best_streak,
                    h_clone.last_completed_date,
                    now_utc,
                    habit_id
                ],
            )?;

            // Record timeline event
            let event_id = format!("timeline-{}", uuid::Uuid::new_v4());
            let meta = serde_json::json!({
                "habit_id": habit_id,
                "streak": h_clone.current_streak,
                "best": h_clone.best_streak
            })
            .to_string();

            conn.execute(
                "INSERT INTO timeline_events (id, event_type, title, metadata_json, created_at) VALUES (?1, 'habit_tick', ?2, ?3, ?4)",
                params![event_id, format!("Habit Maintained: {}", h_clone.title), meta, now_utc],
            )?;

            Ok(())
        },
    )?;

    Ok(habit)
}
