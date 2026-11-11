//! MindSet OS — Hardware & Device Manager Subsystem
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod media_routing;
pub mod network_adapters;
pub mod power_governor;
pub mod usb_and_peripherals;

#[allow(unused_imports)]
pub use media_routing::{MediaDevice, MediaRoutingManager, MediaRoutingReport};
#[allow(unused_imports)]
pub use network_adapters::{DriverResetResult, NetworkAdapterManager, NetworkDiagnosticsReport, WifiAdapterInfo};
#[allow(unused_imports)]
pub use power_governor::{GovernorProfile, PowerGovernorManager, PowerThermalReport};
#[allow(unused_imports)]
pub use usb_and_peripherals::{
    DiskCleanupCategory, DiskCleanupReport, DisplayInfo, PeripheralReport, PeripheralsManager, StorageDrive, UsbDevice,
};

use crate::kernel::MindSetKernel;
use std::sync::Arc;
use tauri::{command, State};

// ---------------------------------------------------------------------------
// 1. Peripheral & Storage Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn devices_get_peripherals() -> Result<PeripheralReport, String> {
    Ok(PeripheralsManager::enumerate_all().await)
}

#[command]
pub async fn devices_eject_drive(mount_point: String) -> Result<String, String> {
    PeripheralsManager::eject_external_drive(&mount_point).await
}

#[command]
pub async fn devices_trigger_disk_cleanup(
    mount_point: Option<String>,
    execute: Option<bool>,
) -> Result<DiskCleanupReport, String> {
    PeripheralsManager::trigger_disk_cleanup(mount_point, execute.unwrap_or(false)).await
}

// ---------------------------------------------------------------------------
// 2. Wi-Fi Adapter & Network Diagnostics Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn devices_get_network_diagnostics() -> Result<NetworkDiagnosticsReport, String> {
    Ok(NetworkAdapterManager::run_diagnostics().await)
}

#[command]
pub async fn devices_reset_wifi_driver(adapter_name: String) -> Result<DriverResetResult, String> {
    Ok(NetworkAdapterManager::reset_driver(&adapter_name).await)
}

#[command]
pub async fn devices_toggle_hotspot(enable: bool, ssid: Option<String>) -> Result<bool, String> {
    NetworkAdapterManager::toggle_hotspot(enable, ssid).await
}

// ---------------------------------------------------------------------------
// 3. Power, Battery & Thermal Governor Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn devices_get_power_thermal_status() -> Result<PowerThermalReport, String> {
    Ok(PowerGovernorManager::get_status().await)
}

#[command]
pub async fn devices_set_power_governor_profile(profile: String) -> Result<PowerThermalReport, String> {
    Ok(PowerGovernorManager::set_profile(&profile).await)
}

// ---------------------------------------------------------------------------
// 4. Media & Hardware Fallback Routing Commands
// ---------------------------------------------------------------------------

#[command]
pub async fn devices_get_media_routing(
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<MediaRoutingReport, String> {
    let diag = kernel.diagnostics.read().await;
    let hw = &diag.hardware;
    Ok(MediaRoutingManager::get_routing_status(hw.has_mic, hw.has_webcam, hw.has_bluetooth).await)
}

#[command]
pub async fn devices_select_media_device(
    kind: String,
    device_id: String,
    kernel: State<'_, Arc<MindSetKernel>>,
) -> Result<MediaRoutingReport, String> {
    MediaRoutingManager::select_active_device(&kind, &device_id).await?;

    let diag = kernel.diagnostics.read().await;
    let hw = &diag.hardware;
    Ok(MediaRoutingManager::get_routing_status(hw.has_mic, hw.has_webcam, hw.has_bluetooth).await)
}
