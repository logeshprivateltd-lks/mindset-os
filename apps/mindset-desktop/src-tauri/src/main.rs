// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod auth;
mod commands;
mod connect;
mod db;
mod devices;
mod fs;
mod kernel;
mod monitor;
mod storage;
mod updater;

use ai::*;
use auth::AuthEngine;
use commands::*;
use connect::*;
use db::DatabaseManager;
use devices::*;
use fs::*;
use monitor::*;
use updater::*;
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

    // Signal SYSTEM_READY to Atomic Rollback Watchdog
    AtomicRollbackManager::signal_system_ready();

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
            db_list_timeline_events,
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
            ai_capture_screen_region,
            ai_capture_active_window,
            ai_extract_screen_text_and_formulas,
            ai_solve_screen_doubt,
            ai_get_voice_capabilities,
            // File Explorer & Storage Operations
            fs_list_directory,
            fs_create_folder,
            fs_rename_item,
            fs_copy_item,
            fs_move_item,
            fs_move_to_recycle_bin,
            fs_restore_from_recycle_bin,
            fs_empty_recycle_bin,
            fs_compress_to_zip,
            fs_extract_archive,
            fs_get_disk_partition_stats,
            fs_read_file_text_or_base64,
            // Zero-RAM RAG Knowledge Commands
            rag_index_document,
            rag_update_last_read_page,
            rag_get_document,
            rag_list_documents,
            rag_search_chunks,
            rag_delete_document,
            // Hardware & Device Manager Commands
            devices_get_peripherals,
            devices_eject_drive,
            devices_trigger_disk_cleanup,
            devices_get_network_diagnostics,
            devices_reset_wifi_driver,
            devices_toggle_hotspot,
            devices_get_power_thermal_status,
            devices_set_power_governor_profile,
            devices_get_media_routing,
            devices_select_media_device,
            // Process & Resource Monitor Commands (/status)
            monitor_get_system_resources,
            monitor_apply_cgroups_limit,
            monitor_remove_cgroups_limit,
            monitor_execute_doctor_action,
            monitor_run_full_system_doctor,
            terminal_execute_command,
            // Atomic OTA Updater & Recovery Commands (/updater & /recovery)
            updater_get_version_info,
            updater_check_for_updates,
            updater_download_and_stage,
            updater_apply_update,
            updater_install_offline_bundle,
            updater_apply_offline_usb_patch,
            updater_get_ostree_status,
            updater_trigger_atomic_rollback,
            recovery_list_snapshots,
            recovery_create_snapshot,
            recovery_restore_snapshot,
            recovery_rollback_update,
            recovery_repair_databases,
            recovery_reset_drivers,
            recovery_reset_network_and_firewall,
            recovery_apply_offline_patch,
            recovery_export_emergency_data_archive,
            recovery_perform_factory_reset,
            // MindSet Connect 6-Module Cross-Device Commands
            connect_generate_pairing_qr,
            connect_list_trusted_devices,
            connect_approve_pairing,
            connect_revoke_device,
            connect_get_sync_status,
            connect_get_active_conflicts,
            connect_resolve_conflict,
            connect_browse_remote_folder,
            connect_preview_remote_file,
            connect_request_remote_delete,
            connect_get_continuity_state,
            connect_broadcast_continuity_state,
            connect_get_active_focus,
            connect_sync_focus_session,
            connect_sync_clipboard,
            connect_send_clicker_action,
            connect_get_hardware_bridge_status,
            connect_toggle_webcam_stream,
            connect_toggle_mic_stream,
            connect_ingest_scanned_document,
            connect_get_security_audit_logs,
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
