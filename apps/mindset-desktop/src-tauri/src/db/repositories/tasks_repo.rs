//! Tasks, Goals & Syllabus Milestones Repository
//! Automatically records chronological timeline events upon task completion
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::hlc_oplog::{execute_with_oplog, HybridLogicalClock};
use crate::kernel::{EventBus, KernelEvent};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOrGoal {
    pub id: String,
    pub parent_goal_id: Option<String>,
    pub title: String,
    pub subject_tag: Option<String>,
    pub priority: i64,
    pub status: String,
    pub target_score_weight: f64,
    pub scheduled_start: Option<String>,
    pub scheduled_end: Option<String>,
    pub completed_at: Option<String>,
    pub xp_reward: i64,
    pub updated_at: String,
}

pub fn list_tasks(
    conn: &Connection,
    status_filter: Option<String>,
    priority_filter: Option<i64>,
) -> Result<Vec<TaskOrGoal>, rusqlite::Error> {
    let mut sql = "SELECT id, parent_goal_id, title, subject_tag, priority, status, target_score_weight, scheduled_start, scheduled_end, completed_at, xp_reward, updated_at FROM tasks_and_goals WHERE 1=1".to_string();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(s) = status_filter {
        sql.push_str(" AND status = ?");
        params.push(Box::new(s));
    }
    if let Some(p) = priority_filter {
        sql.push_str(" AND priority = ?");
        params.push(Box::new(p));
    }
    sql.push_str(" ORDER BY priority ASC, scheduled_start ASC");

    let mut stmt = conn.prepare(&sql)?;
    let mut tasks = Vec::new();

    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut rows = stmt.query(params_refs.as_slice())?;

    while let Some(row) = rows.next()? {
        tasks.push(TaskOrGoal {
            id: row.get(0)?,
            parent_goal_id: row.get(1)?,
            title: row.get(2)?,
            subject_tag: row.get(3)?,
            priority: row.get(4)?,
            status: row.get(5)?,
            target_score_weight: row.get(6)?,
            scheduled_start: row.get(7)?,
            scheduled_end: row.get(8)?,
            completed_at: row.get(9)?,
            xp_reward: row.get(10)?,
            updated_at: row.get(11)?,
        });
    }

    Ok(tasks)
}

pub fn create_task(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut task: TaskOrGoal,
) -> Result<TaskOrGoal, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    task.updated_at = now.clone();
    if task.id.is_empty() {
        task.id = format!("task-{}", uuid::Uuid::new_v4());
    }

    let payload_json = serde_json::to_string(&task).unwrap_or_default();
    let t_clone = task.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "tasks_and_goals",
        &t_clone.id,
        "INSERT",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                INSERT INTO tasks_and_goals (
                    id, parent_goal_id, title, subject_tag, priority, status, 
                    target_score_weight, scheduled_start, scheduled_end, completed_at, 
                    xp_reward, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                "#,
                params![
                    t_clone.id,
                    t_clone.parent_goal_id,
                    t_clone.title,
                    t_clone.subject_tag,
                    t_clone.priority,
                    t_clone.status,
                    t_clone.target_score_weight,
                    t_clone.scheduled_start,
                    t_clone.scheduled_end,
                    t_clone.completed_at,
                    t_clone.xp_reward,
                    t_clone.updated_at
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(task)
}

pub fn update_task(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    mut task: TaskOrGoal,
) -> Result<TaskOrGoal, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    task.updated_at = now.clone();

    let payload_json = serde_json::to_string(&task).unwrap_or_default();
    let t_clone = task.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "tasks_and_goals",
        &t_clone.id,
        "UPDATE",
        &payload_json,
        |conn| {
            conn.execute(
                r#"
                UPDATE tasks_and_goals SET
                    parent_goal_id = ?1,
                    title = ?2,
                    subject_tag = ?3,
                    priority = ?4,
                    status = ?5,
                    target_score_weight = ?6,
                    scheduled_start = ?7,
                    scheduled_end = ?8,
                    completed_at = ?9,
                    xp_reward = ?10,
                    updated_at = ?11
                WHERE id = ?12
                "#,
                params![
                    t_clone.parent_goal_id,
                    t_clone.title,
                    t_clone.subject_tag,
                    t_clone.priority,
                    t_clone.status,
                    t_clone.target_score_weight,
                    t_clone.scheduled_start,
                    t_clone.scheduled_end,
                    t_clone.completed_at,
                    t_clone.xp_reward,
                    t_clone.updated_at,
                    t_clone.id
                ],
            )?;
            Ok(())
        },
    )?;

    Ok(task)
}

pub fn delete_task(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    task_id: &str,
) -> Result<(), rusqlite::Error> {
    let payload_json = serde_json::json!({ "id": task_id }).to_string();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "tasks_and_goals",
        task_id,
        "DELETE",
        &payload_json,
        |conn| {
            conn.execute("DELETE FROM tasks_and_goals WHERE id = ?1", params![task_id])?;
            Ok(())
        },
    )
}

pub fn complete_task(
    core_conn: &Connection,
    oplog_conn: &Connection,
    hlc: &Arc<Mutex<HybridLogicalClock>>,
    bus: Option<&Arc<EventBus>>,
    task_id: &str,
) -> Result<TaskOrGoal, rusqlite::Error> {
    let now = Utc::now().to_rfc3339();

    // 1. Fetch current task
    let mut task: TaskOrGoal = core_conn.query_row(
        r#"
        SELECT id, parent_goal_id, title, subject_tag, priority, status, 
               target_score_weight, scheduled_start, scheduled_end, completed_at, 
               xp_reward, updated_at
        FROM tasks_and_goals WHERE id = ?1
        "#,
        params![task_id],
        |row| {
            Ok(TaskOrGoal {
                id: row.get(0)?,
                parent_goal_id: row.get(1)?,
                title: row.get(2)?,
                subject_tag: row.get(3)?,
                priority: row.get(4)?,
                status: row.get(5)?,
                target_score_weight: row.get(6)?,
                scheduled_start: row.get(7)?,
                scheduled_end: row.get(8)?,
                completed_at: row.get(9)?,
                xp_reward: row.get(10)?,
                updated_at: row.get(11)?,
            })
        },
    )?;

    task.status = "completed".to_string();
    task.completed_at = Some(now.clone());
    task.updated_at = now.clone();

    let payload_json = serde_json::to_string(&task).unwrap_or_default();
    let t_clone = task.clone();

    execute_with_oplog(
        core_conn,
        oplog_conn,
        hlc,
        bus,
        "tasks_and_goals",
        &t_clone.id,
        "UPDATE",
        &payload_json,
        |conn| {
            conn.execute(
                "UPDATE tasks_and_goals SET status = 'completed', completed_at = ?1, updated_at = ?2 WHERE id = ?3",
                params![now, now, task_id],
            )?;

            // Auto-append into chronological timeline_events
            let event_id = format!("timeline-{}", uuid::Uuid::new_v4());
            let meta = serde_json::json!({
                "task_id": t_clone.id,
                "xp_reward": t_clone.xp_reward,
                "subject": t_clone.subject_tag,
            }).to_string();

            conn.execute(
                "INSERT INTO timeline_events (id, event_type, title, metadata_json, created_at) VALUES (?1, 'task_completion', ?2, ?3, ?4)",
                params![event_id, format!("Completed Objective: {}", t_clone.title), meta, now],
            )?;

            Ok(())
        },
    )?;

    if let Some(event_bus) = bus {
        event_bus.publish(KernelEvent::TaskCompleted {
            task_id: task_id.to_string(),
            xp_earned: task.xp_reward as u32,
        });
    }

    Ok(task)
}
