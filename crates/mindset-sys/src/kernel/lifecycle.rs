//! MindSet Kernel Boot Sequencer & Deterministic Startup Lifecycle
//! Sequence: BOOT_INIT -> DB_MIGRATE -> HARDWARE_PROBE -> SERVICE_REGISTER -> SYSTEM_READY
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::DatabaseManager;
use crate::kernel::event_bus::{EventBus, HardwareCapabilities, KernelEvent};
use crate::kernel::registry::{
    AuthService, HardwareProbeService, KernelError, MemoryWatchdogService, OllamaHealthService,
    SchedulerService, ServiceRegistry, StorageService,
};
use crate::storage::MindSetPaths;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootPhase {
    BootInit,
    DbMigrate,
    HardwareProbe,
    ServiceRegister,
    SystemReady,
}

impl BootPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BootInit => "BOOT_INIT",
            Self::DbMigrate => "DB_MIGRATE",
            Self::HardwareProbe => "HARDWARE_PROBE",
            Self::ServiceRegister => "SERVICE_REGISTER",
            Self::SystemReady => "SYSTEM_READY",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDuration {
    pub boot_init_ms: u64,
    pub db_migrate_ms: u64,
    pub hardware_probe_ms: u64,
    pub service_register_ms: u64,
    pub total_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootDiagnostics {
    pub phase: BootPhase,
    pub boot_duration_ms: u64,
    pub stage_durations: StageDuration,
    pub hardware: HardwareCapabilities,
    pub timestamp_utc: String,
}

pub struct KernelBootSequencer;

impl KernelBootSequencer {
    /// Execute the complete 5-stage deterministic startup lifecycle with millisecond telemetry
    pub async fn execute(
        bus: Arc<EventBus>,
    ) -> Result<
        (
            MindSetPaths,
            DatabaseManager,
            ServiceRegistry,
            BootDiagnostics,
        ),
        KernelError,
    > {
        let total_start = Instant::now();

        // -------------------------------------------------------------
        // Phase 1: BOOT_INIT (Verify ~/.mindset_data/ layout)
        // -------------------------------------------------------------
        let t1 = Instant::now();
        info!("[LIFECYCLE 1/5] Executing BOOT_INIT...");
        let paths =
            MindSetPaths::resolve().map_err(|e| KernelError::ServiceStart(e.to_string()))?;
        paths
            .initialize_filesystem()
            .map_err(|e| KernelError::ServiceStart(e.to_string()))?;
        let boot_init_ms = t1.elapsed().as_millis() as u64;

        // -------------------------------------------------------------
        // Phase 2: DB_MIGRATE (SQLite WAL & Low-RAM Pragmas)
        // -------------------------------------------------------------
        let t2 = Instant::now();
        info!("[LIFECYCLE 2/5] Executing DB_MIGRATE...");
        let db = DatabaseManager::initialize(&paths)
            .map_err(|e| KernelError::ServiceStart(e.to_string()))?;
        let db_migrate_ms = t2.elapsed().as_millis() as u64;

        // -------------------------------------------------------------
        // Phase 3: HARDWARE_PROBE (Sensory Peripheral & USB Wi-Fi Detection)
        // -------------------------------------------------------------
        let t3 = Instant::now();
        info!("[LIFECYCLE 3/5] Executing HARDWARE_PROBE...");
        let hardware = HardwareProbeService::detect_hardware();
        let hardware_probe_ms = t3.elapsed().as_millis() as u64;

        // -------------------------------------------------------------
        // Phase 4: SERVICE_REGISTER (Background Micro-Service Daemons)
        // -------------------------------------------------------------
        let t4 = Instant::now();
        info!("[LIFECYCLE 4/5] Executing SERVICE_REGISTER...");
        let mut registry = ServiceRegistry::new();
        registry.register(Arc::new(Mutex::new(SchedulerService::new())));
        registry.register(Arc::new(Mutex::new(MemoryWatchdogService::new(80.0))));
        registry.register(Arc::new(Mutex::new(HardwareProbeService::new())));
        registry.register(Arc::new(Mutex::new(OllamaHealthService::new(
            "http://127.0.0.1:11434/api/tags",
        ))));
        registry.register(Arc::new(Mutex::new(AuthService::new(5))));
        registry.register(Arc::new(Mutex::new(StorageService::new())));

        registry.start_all(Arc::clone(&bus)).await?;
        let service_register_ms = t4.elapsed().as_millis() as u64;

        // -------------------------------------------------------------
        // Phase 5: SYSTEM_READY (Broadcast event & unlock shell)
        // -------------------------------------------------------------
        info!("[LIFECYCLE 5/5] Reached SYSTEM_READY state.");
        let total_boot_ms = total_start.elapsed().as_millis() as u64;

        let stage_durations = StageDuration {
            boot_init_ms,
            db_migrate_ms,
            hardware_probe_ms,
            service_register_ms,
            total_ms: total_boot_ms,
        };

        let diagnostics = BootDiagnostics {
            phase: BootPhase::SystemReady,
            boot_duration_ms: total_boot_ms,
            stage_durations,
            hardware: hardware.clone(),
            timestamp_utc: Utc::now().to_rfc3339(),
        };

        bus.publish(KernelEvent::SystemReady {
            uptime_ms: total_boot_ms,
            hardware,
        });

        info!(
            "MindSet OS Kernel ready in {}ms [Init: {}ms, DB: {}ms, HW: {}ms, Svc: {}ms]",
            total_boot_ms, boot_init_ms, db_migrate_ms, hardware_probe_ms, service_register_ms
        );

        Ok((paths, db, registry, diagnostics))
    }
}
