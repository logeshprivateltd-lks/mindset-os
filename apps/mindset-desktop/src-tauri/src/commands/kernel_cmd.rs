//! MindSet OS Kernel IPC Command Handlers
//! Real-time kernel diagnostics, service controls, emergency purge, and event stream
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::kernel::{
    EmergencyPurgeResult, KernelEvent, KernelStatusResponse, MindSetKernel, ServiceHealth,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_kernel_status(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<KernelStatusResponse, String> {
    Ok(kernel.get_status().await)
}

#[tauri::command]
pub async fn control_kernel_service(
    service_name: String,
    action: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<ServiceHealth, String> {
    kernel
        .control_service(&service_name, &action)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn trigger_emergency_ram_purge(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<EmergencyPurgeResult, String> {
    Ok(kernel.trigger_emergency_purge().await)
}

#[tauri::command]
pub async fn get_kernel_event_history(
    limit: Option<usize>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<Vec<KernelEvent>, String> {
    Ok(kernel.get_event_history(limit.unwrap_or(50)))
}

#[tauri::command]
pub async fn kernel_execute_power_action(
    action: String,
    reason: Option<String>,
    kernel: State<'_, Arc<MindSetKernel>>,
    app_handle: tauri::AppHandle,
) -> Result<crate::kernel::power::PowerActionResult, String> {
    let act = crate::kernel::power::PowerAction::from_str(&action)
        .ok_or_else(|| format!("Unknown power action: {}", action))?;
    crate::kernel::power::PowerManager::execute_power_action(
        act,
        reason,
        kernel.inner().clone(),
        Some(app_handle),
    )
    .await
}

#[tauri::command]
pub async fn kernel_get_power_state(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<crate::kernel::power::PowerStateResponse, String> {
    Ok(crate::kernel::power::PowerManager::get_power_state(kernel.inner()))
}

#[tauri::command]
pub async fn kernel_save_wallpaper_asset(
    source_path: String,
    filename: Option<String>,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<crate::kernel::power::WallpaperAssetResponse, String> {
    crate::kernel::power::PowerManager::save_wallpaper_asset(
        &kernel.paths,
        &source_path,
        filename,
    )
}

