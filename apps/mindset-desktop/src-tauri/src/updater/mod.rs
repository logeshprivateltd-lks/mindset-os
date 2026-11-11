//! MindSet OS — Atomic OTA Updater & Recovery Subsystem
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod atomic_rollback;
pub mod atomic_updater;
pub mod ota_updater;
pub mod recovery_engine;

#[allow(unused_imports)]
pub use atomic_rollback::{AtomicRollbackManager, OstreeDeploymentInfo, RollbackExecutionResult};
#[allow(unused_imports)]
pub use atomic_updater::{
    AtomicUpdater, OfflineBundleManifest, OSVersionInfo, UpdateChannel, UpdatePackageInfo,
    UpdateStageProgress, CURRENT_OS_VERSION, LKS_ED25519_ROOT_FINGERPRINT,
};
#[allow(unused_imports)]
pub use ota_updater::{
    OfflinePatchResult, OSVersionDetails, OtaUpdater, PatchChannel, PatchPackageInfo, PatchProgress,
    LKS_ED25519_ROOT_KEY,
};
#[allow(unused_imports)]
pub use recovery_engine::{
    DatabaseRepairReport, DriverResetReport, RecoveryActionResult, RecoveryEngine, SystemSnapshot,
};

use crate::AppState;
use tauri::{command, State};

// ---------------------------------------------------------------------------
// 1. Atomic OTA Updater Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn updater_get_version_info() -> Result<OSVersionInfo, String> {
    let updater = AtomicUpdater::default_instance();
    Ok(updater.get_version_info())
}

#[command]
pub async fn updater_check_for_updates(channel: String) -> Result<Vec<UpdatePackageInfo>, String> {
    let updater = AtomicUpdater::default_instance();
    Ok(updater.check_for_updates(&channel))
}

#[command]
pub async fn updater_download_and_stage(update_id: String) -> Result<UpdateStageProgress, String> {
    let updater = AtomicUpdater::default_instance();
    updater.download_and_stage(&update_id)
}

#[command]
pub async fn updater_apply_update(
    update_id: String,
    state: State<'_, AppState>,
) -> Result<UpdateStageProgress, String> {
    let updater = AtomicUpdater::default_instance();
    updater.apply_update(&update_id, state.db.clone())
}

#[command]
pub async fn updater_install_offline_bundle(
    bundle_path: String,
) -> Result<OfflineBundleManifest, String> {
    let updater = AtomicUpdater::default_instance();
    updater.install_offline_bundle(&bundle_path)
}

#[command]
pub async fn updater_apply_offline_usb_patch(
    bundle_path: String,
) -> Result<OfflinePatchResult, String> {
    let ota = OtaUpdater::default_instance();
    ota.apply_offline_usb_patch(&bundle_path)
}

#[command]
pub async fn updater_get_ostree_status() -> Result<OstreeDeploymentInfo, String> {
    let rollback_mgr = AtomicRollbackManager::default_instance();
    Ok(rollback_mgr.query_ostree_status())
}

#[command]
pub async fn updater_trigger_atomic_rollback() -> Result<RollbackExecutionResult, String> {
    let rollback_mgr = AtomicRollbackManager::default_instance();
    rollback_mgr.execute_atomic_rollback()
}

// ---------------------------------------------------------------------------
// 2. Standalone Recovery Mode Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn recovery_list_snapshots() -> Result<Vec<SystemSnapshot>, String> {
    let engine = RecoveryEngine::default_instance();
    Ok(engine.list_snapshots())
}

#[command]
pub async fn recovery_create_snapshot(
    label: String,
    is_automatic: bool,
    state: State<'_, AppState>,
) -> Result<SystemSnapshot, String> {
    let engine = RecoveryEngine::default_instance();
    engine.create_snapshot(&label, is_automatic, state.db.clone())
}

#[command]
pub async fn recovery_restore_snapshot(
    snapshot_id: String,
    state: State<'_, AppState>,
) -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.restore_snapshot(&snapshot_id, state.db.clone())
}

#[command]
pub async fn recovery_rollback_update() -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.rollback_latest_update()
}

#[command]
pub async fn recovery_repair_databases(
    state: State<'_, AppState>,
) -> Result<DatabaseRepairReport, String> {
    let engine = RecoveryEngine::default_instance();
    engine.repair_sqlite_databases(state.db.clone())
}

#[command]
pub async fn recovery_reset_drivers() -> Result<DriverResetReport, String> {
    let engine = RecoveryEngine::default_instance();
    engine.reset_hardware_drivers()
}

#[command]
pub async fn recovery_reset_network_and_firewall() -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.reset_network_and_firewall()
}

#[command]
pub async fn recovery_apply_offline_patch(
    bundle_path: String,
) -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.apply_offline_security_patch(&bundle_path)
}

#[command]
pub async fn recovery_export_emergency_data_archive(
    target_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.export_emergency_data_archive(target_path, state.db.clone())
}

#[command]
pub async fn recovery_perform_factory_reset(
    confirmation_token: String,
    state: State<'_, AppState>,
) -> Result<RecoveryActionResult, String> {
    let engine = RecoveryEngine::default_instance();
    engine.perform_factory_reset(&confirmation_token, state.db.clone())
}
