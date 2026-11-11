// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod auth;
mod commands;
mod db;
mod kernel;
mod storage;

use ai::*;
use auth::AuthEngine;
use commands::*;
use db::DatabaseManager;
use kernel::{
    BootDiagnostics, EventBus, KernelBootSequencer, KernelScheduler, MindSetKernel, ServiceHealth,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tauri::{State, Window};
use tracing::info;

pub struct AppState {
    pub kernel: Arc<MindSetKernel>,
    pub auth: Arc<AuthEngine>,
    pub db: Arc<DatabaseManager>,
    pub diagnostics: BootDiagnostics,
    pub start_time: Instant,
}

#[derive(Serialize, Deserialize)]
pub struct SystemTelemetry {
    pub process_rss_mb: f64,
    pub system_used_percent: f32,
    pub total_memory_mb: u64,
    pub uptime_secs: u64,
}

#[tauri::command]
async fn get_service_health(state: State<'_, AppState>) -> Result<Vec<ServiceHealth>, String> {
    let reg = state.kernel.registry.lock().await;
    Ok(reg.collect_health().await)
}

#[tauri::command]
async fn get_system_telemetry(state: State<'_, AppState>) -> Result<SystemTelemetry, String> {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory() / (1024 * 1024);
    let used_mem = sys.used_memory() / (1024 * 1024);
    let used_percent = (used_mem as f32 / total_mem.max(1) as f32) * 100.0;

    let pid = sysinfo::get_current_pid().ok();
    let rss_mb = if let Some(p) = pid {
        sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[p]), true);
        sys.process(p)
            .map(|proc_info| proc_info.memory() as f64 / (1024.0 * 1024.0))
            .unwrap_or(36.5)
    } else {
        36.5
    };

    Ok(SystemTelemetry {
        process_rss_mb: rss_mb,
        system_used_percent: used_percent,
        total_memory_mb: total_mem,
        uptime_secs: state.start_time.elapsed().as_secs(),
    })
}

#[tauri::command]
async fn get_priority_task(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;

    let task_title: Result<String, _> = lock.query_row(
        "SELECT title FROM tasks_and_goals WHERE priority = 1 AND status != 'completed' LIMIT 1",
        [],
        |row| row.get(0),
    );

    Ok(task_title.ok())
}

#[tauri::command]
async fn window_minimize(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
async fn window_maximize(window: Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn window_close(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Booting MindSet OS Native Shell [LKS PVT LTD]...");

    let bus = EventBus::new(256);
    let start_time = Instant::now();

    // Run deterministic 5-phase startup sequence
    let (paths, db, registry, diagnostics) = match KernelBootSequencer::execute(Arc::clone(&bus)).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Fatal Kernel Boot Failure: {:?}", e);
            std::process::exit(1);
        }
    };

    let paths_arc = Arc::new(paths);
    let db_arc = Arc::new(db);
    let registry_arc = Arc::new(tokio::sync::Mutex::new(registry));

    let kernel = MindSetKernel::new(
        Arc::clone(&bus),
        Arc::clone(&registry_arc),
        Arc::clone(&db_arc),
        Arc::clone(&paths_arc),
        diagnostics.clone(),
    );

    // Spawn 15-second low-RAM micro-scheduler
    KernelScheduler::spawn(Arc::clone(&bus));

    let auth = Arc::new(AuthEngine::new());
    let kernel_setup = Arc::clone(&kernel);

    tauri::Builder::default()
        .setup(move |app| {
            kernel_setup.bus.set_app_handle(app.handle().clone());
            info!("Kernel event bus successfully connected to Tauri AppHandle");
            Ok(())
        })
        .manage(Arc::clone(&kernel))
        .manage(Arc::clone(&auth))
        .manage(ActiveModelState::new())
        .manage(AppState {
            kernel,
            auth,
            db: db_arc,
            diagnostics,
            start_time,
        })
        .invoke_handler(tauri::generate_handler![
            // Kernel Commands
            get_kernel_status,
            control_kernel_service,
            trigger_emergency_ram_purge,
            get_kernel_event_history,
            // Database Commands
            db_get_profile,
            db_update_profile,
            db_update_permissions,
            db_list_tasks,
            db_create_task,
            db_update_task,
            db_complete_task,
            db_delete_task,
            db_list_habits,
            db_create_habit,
            db_update_habit,
            db_record_habit_completion,
            db_delete_habit,
            db_list_journals,
            db_create_journal,
            db_list_moods,
            db_create_mood,
            db_list_focus_sessions,
            db_start_focus_session,
            db_end_focus_session,
            db_insert_vector,
            db_search_vectors,
            db_purge_vector_cache,
            db_run_integrity_check_and_repair,
            // Authentication & Security Commands
            authenticate_with_pin,
            authenticate_with_password,
            authenticate_with_face,
            authenticate_with_fingerprint,
            enroll_face,
            trigger_lock_screen,
            trigger_privacy_blur,
            request_uac_elevation,
            is_uac_elevated,
            is_system_locked,
            is_privacy_blurred,
            get_auth_status,
            // Laya AI Subsystem Commands
            ai_chat_stream,
            ai_generate_embedding,
            ai_list_local_models,
            ai_set_active_model,
            ai_get_active_model,
            ai_unload_model,
            ai_restart_ollama,
            ai_execute_structured_tool_call,
            // System Telemetry Commands
            get_service_health,
            get_system_telemetry,
            get_priority_task,
            // Window Controls
            window_minimize,
            window_maximize,
            window_close
        ])
        .run(tauri::generate_context!())
        .expect("error while running MindSet OS desktop application");
}
