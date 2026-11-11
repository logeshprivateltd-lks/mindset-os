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

