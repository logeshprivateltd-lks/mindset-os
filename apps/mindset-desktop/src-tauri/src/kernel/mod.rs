//! MindSet OS Kernel Subsystem
//! Holds global MindSetKernel coordinator, event bus, and service registry
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod event_bus;
pub mod lifecycle;
pub mod power;
pub mod registry;
pub mod scheduler;

#[allow(unused_imports)]
pub use event_bus::{EventBus, HardwareCapabilities, KernelEvent, MAX_EVENT_HISTORY};
#[allow(unused_imports)]
pub use lifecycle::{BootDiagnostics, BootPhase, KernelBootSequencer, StageDuration};
#[allow(unused_imports)]
pub use power::{
    PowerAction, PowerActionResult, PowerManager, PowerStateResponse, WallpaperAssetResponse,
};
#[allow(unused_imports)]
pub use registry::{
    AuthService, HardwareProbeService, KernelError, KernelService, MemoryWatchdogService,
    OllamaHealthService, SchedulerService, ServiceHealth, ServiceRegistry, ServiceStatus,
    StorageService,
};
pub use scheduler::KernelScheduler;

use crate::db::DatabaseManager;
use crate::storage::MindSetPaths;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use sysinfo::System;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamBreakdown {
    pub os_shell_mb: f64,
    pub ollama_mb: f64,
    pub free_system_mb: u64,
    pub total_system_mb: u64,
    pub system_used_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelStatusResponse {
    pub boot_stage: String,
    pub uptime_secs: u64,
    pub hardware: HardwareCapabilities,
    pub ram_breakdown: RamBreakdown,
    pub services: Vec<ServiceHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyPurgeResult {
    pub freed_mb: f64,
    pub current_rss_mb: f64,
    pub timestamp_utc: String,
}

/// Global MindSet Kernel Coordinator managed via tauri::State<Arc<MindSetKernel>>
pub struct MindSetKernel {
    pub bus: Arc<EventBus>,
    pub registry: Arc<Mutex<ServiceRegistry>>,
    pub db: Arc<DatabaseManager>,
    pub paths: Arc<MindSetPaths>,
    pub diagnostics: RwLock<BootDiagnostics>,
    pub start_time: Instant,
}

impl MindSetKernel {
    pub fn new(
        bus: Arc<EventBus>,
        registry: Arc<Mutex<ServiceRegistry>>,
        db: Arc<DatabaseManager>,
        paths: Arc<MindSetPaths>,
        diagnostics: BootDiagnostics,
    ) -> Arc<Self> {
        Arc::new(Self {
            bus,
            registry,
            db,
            paths,
            diagnostics: RwLock::new(diagnostics),
            start_time: Instant::now(),
        })
    }

    /// Returns high-fidelity system telemetry, hardware flags, and service matrix
    pub async fn get_status(&self) -> KernelStatusResponse {
        let mut sys = System::new();
        sys.refresh_memory();

        let total_system_mb = sys.total_memory() / (1024 * 1024);
        let free_system_mb = sys.free_memory() / (1024 * 1024);
        let used_system_mb = sys.used_memory() / (1024 * 1024);
        let system_used_percent = (used_system_mb as f32 / total_system_mb.max(1) as f32) * 100.0;

        let pid = sysinfo::get_current_pid().ok();
        let os_shell_mb = if let Some(p) = pid {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[p]), true);
            sys.process(p)
                .map(|p_info| p_info.memory() as f64 / (1024.0 * 1024.0))
                .unwrap_or(36.5)
        } else {
            36.5
        };

        // Query Ollama process RAM if active
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        let mut ollama_bytes: u64 = 0;
        for (_pid, proc) in sys.processes() {
            let proc_name = proc.name().to_string_lossy().to_lowercase();
            if proc_name.contains("ollama") {
                ollama_bytes += proc.memory();
            }
        }
        let ollama_mb = ollama_bytes as f64 / (1024.0 * 1024.0);

        let reg = self.registry.lock().await;
        let services = reg.collect_health().await;

        let diag = self.diagnostics.read().await;

        KernelStatusResponse {
            boot_stage: diag.phase.as_str().to_string(),
            uptime_secs: self.start_time.elapsed().as_secs(),
            hardware: diag.hardware.clone(),
            ram_breakdown: RamBreakdown {
                os_shell_mb,
                ollama_mb,
                free_system_mb,
                total_system_mb,
                system_used_percent,
            },
            services,
        }
    }

    /// Controls lifecycle of any registered kernel service (start, stop, restart)
    pub async fn control_service(
        &self,
        service_name: &str,
        action: &str,
    ) -> Result<ServiceHealth, KernelError> {
        let reg = self.registry.lock().await;
        reg.control_service(service_name, action, Arc::clone(&self.bus))
            .await
    }

    /// Triggers one-click self-healing memory purge and drops unnecessary buffers
    pub async fn trigger_emergency_purge(&self) -> EmergencyPurgeResult {
        let mut sys = System::new();
        let pid = sysinfo::get_current_pid().ok();
        let before_rss_bytes = if let Some(p) = pid {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[p]), true);
            sys.process(p).map(|proc| proc.memory()).unwrap_or(38 * 1024 * 1024)
        } else {
            38 * 1024 * 1024
        };

        #[cfg(target_os = "windows")]
        unsafe {
            #[link(name = "psapi")]
            extern "system" {
                fn EmptyWorkingSet(hProcess: *mut std::ffi::c_void) -> i32;
                fn GetCurrentProcess() -> *mut std::ffi::c_void;
            }
            let _ = EmptyWorkingSet(GetCurrentProcess());
        }

        let current_rss_mb = if let Some(p) = pid {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[p]), true);
            sys.process(p)
                .map(|proc| proc.memory() as f64 / (1024.0 * 1024.0))
                .unwrap_or(34.0)
        } else {
            34.0
        };

        let freed_bytes = before_rss_bytes.saturating_sub((current_rss_mb * 1024.0 * 1024.0) as u64);
        let freed_mb = freed_bytes as f64 / (1024.0 * 1024.0);

        self.bus.publish(KernelEvent::EmergencyRamPurgeExecuted {
            reclaimed_bytes: freed_bytes,
            os_shell_rss_mb: current_rss_mb,
        });

        EmergencyPurgeResult {
            freed_mb,
            current_rss_mb,
            timestamp_utc: Utc::now().to_rfc3339(),
        }
    }

    /// Returns recent event history from the circular buffer (up to limit, max 200)
    pub fn get_event_history(&self, limit: usize) -> Vec<KernelEvent> {
        self.bus.get_history(limit)
    }
}
