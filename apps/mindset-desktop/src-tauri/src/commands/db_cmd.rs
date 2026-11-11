//! MindSet Database IPC Command Handlers
//! Strongly typed query and mutation dispatchers for local_core and sync_oplog
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::repositories::{
    diagnostics_repo::{self, DbIntegrityReport},
    habits_repo::{self, Habit},
    profile_repo::{self, UserProfile},
    tasks_repo::{self, TaskOrGoal},
    vector_repo::{self, VectorSearchResult},
    wellbeing_repo::{self, FocusSession, JournalEntry, MoodCheckin},
};
use crate::kernel::MindSetKernel;
use std::sync::Arc;
use tauri::State;

// ---------------------------------------------------------------------------
// 1. Profile Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_get_profile(
    user_id: Option<String>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Option<UserProfile>, String> {
    let uid = user_id.unwrap_or_else(|| "default_student".to_string());
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    profile_repo::get_profile(&lock, &uid).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_profile(
    profile: UserProfile,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<UserProfile, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    profile_repo::update_profile(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        profile,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_permissions(
    user_id: String,
    permissions_json: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<(), String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    profile_repo::update_permissions(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &user_id,
        &permissions_json,
    )
    .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// 2. Tasks & Goals Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_list_tasks(
    status: Option<String>,
    priority: Option<i64>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<TaskOrGoal>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    tasks_repo::list_tasks(&lock, status, priority).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_task(
    task: TaskOrGoal,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<TaskOrGoal, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    tasks_repo::create_task(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        task,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_task(
    task: TaskOrGoal,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<TaskOrGoal, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    tasks_repo::update_task(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        task,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_complete_task(
    task_id: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<TaskOrGoal, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    tasks_repo::complete_task(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &task_id,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_delete_task(
    task_id: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<(), String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    tasks_repo::delete_task(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &task_id,
    )
    .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// 3. Habits Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_list_habits(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<Habit>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    habits_repo::list_habits(&lock).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_habit(
    habit: Habit,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Habit, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    habits_repo::create_habit(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        habit,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_habit(
    habit: Habit,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Habit, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    habits_repo::update_habit(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        habit,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_record_habit_completion(
    habit_id: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Habit, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    habits_repo::record_habit_completion(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &habit_id,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_delete_habit(
    habit_id: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<(), String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    habits_repo::delete_habit(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &habit_id,
    )
    .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// 4. Wellbeing Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_list_journals(
    limit: Option<usize>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<JournalEntry>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    wellbeing_repo::list_journals(&lock, limit.unwrap_or(50)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_journal(
    entry: JournalEntry,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<JournalEntry, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    wellbeing_repo::create_journal(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        entry,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_list_moods(
    limit: Option<usize>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<MoodCheckin>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    wellbeing_repo::list_moods(&lock, limit.unwrap_or(50)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_mood(
    checkin: MoodCheckin,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<MoodCheckin, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    wellbeing_repo::create_mood(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        checkin,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_list_focus_sessions(
    limit: Option<usize>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<FocusSession>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    wellbeing_repo::list_focus_sessions(&lock, limit.unwrap_or(50)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_start_focus_session(
    session: FocusSession,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<FocusSession, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    wellbeing_repo::start_focus_session(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        session,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_end_focus_session(
    session_id: String,
    actual_minutes: i64,
    interruption_count: i64,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<(), String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    wellbeing_repo::end_focus_session(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &session_id,
        actual_minutes,
        interruption_count,
    )
    .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// 5. Vector Search Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_insert_vector(
    id: String,
    source_type: String,
    source_id: String,
    embedding: Vec<f32>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<(), String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    vector_repo::insert_vector_embedding(&lock, &id, &source_type, &source_id, &embedding)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_search_vectors(
    query_vec: Vec<f32>,
    top_k: usize,
    source_filter: Option<String>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<VectorSearchResult>, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    vector_repo::search_similar_vectors(&lock, &query_vec, top_k, source_filter)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_purge_vector_cache(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<usize, String> {
    let conn = kernel.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    vector_repo::purge_vector_cache(&lock).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// 6. Diagnostics & Integrity Doctor
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn db_run_integrity_check_and_repair(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<DbIntegrityReport, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    diagnostics_repo::run_db_integrity_check_and_repair(
        &core_lock,
        &oplog_lock,
        &kernel.db.core_path,
        &kernel.db.oplog_path,
    )
    .map_err(|e| e.to_string())
}
