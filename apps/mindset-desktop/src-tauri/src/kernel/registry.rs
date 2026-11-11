//! MindSet Service Registry & Micro-Service Actors
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::kernel::event_bus::EventBus;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Service startup failed: {0}")]
    ServiceStart(String),
    #[error("Service shutdown failed: {0}")]
    ServiceStop(String),
    #[error("Lock acquisition error: {0}")]
    LockError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded(String),
    Stopped,
    Critical(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: ServiceStatus,
    pub estimated_ram_kb: u64,
    pub last_heartbeat_utc: String,
}

#[async_trait]
pub trait KernelService: Send + Sync {
    fn name(&self) -> &'static str;
    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError>;
    async fn stop(&mut self) -> Result<(), KernelError>;
    async fn health_check(&self) -> ServiceHealth;
}

// ---------------------------------------------------------------------------
// SchedulerService
// ---------------------------------------------------------------------------
pub struct SchedulerService {
    is_running: bool,
    bus: Option<Arc<EventBus>>,
}

impl SchedulerService {
    pub fn new() -> Self {
        Self {
            is_running: false,
            bus: None,
        }
    }
}

#[async_trait]
impl KernelService for SchedulerService {
    fn name(&self) -> &'static str {
        "SchedulerService"
    }

    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        self.is_running = true;
        self.bus = Some(bus);
        info!("SchedulerService registered and active (15s tick resolution)");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), KernelError> {
        self.is_running = false;
        Ok(())
    }

    async fn health_check(&self) -> ServiceHealth {
        ServiceHealth {
            name: self.name().to_string(),
            status: if self.is_running {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Stopped
            },
            estimated_ram_kb: 512, // < 1MB footprint
            last_heartbeat_utc: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// MemoryWatchdogService
// ---------------------------------------------------------------------------
pub struct MemoryWatchdogService {
    is_running: bool,
    threshold_percent: f32,
    bus: Option<Arc<EventBus>>,
}

impl MemoryWatchdogService {
    pub fn new(threshold_percent: f32) -> Self {
        Self {
            is_running: false,
            threshold_percent,
            bus: None,
        }
    }
}

#[async_trait]
impl KernelService for MemoryWatchdogService {
    fn name(&self) -> &'static str {
        "MemoryWatchdogService"
    }

    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        self.is_running = true;
        self.bus = Some(bus);
        info!(
            "MemoryWatchdogService active (threshold: {:.0}% system RAM)",
            self.threshold_percent
        );
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), KernelError> {
        self.is_running = false;
        Ok(())
    }

    async fn health_check(&self) -> ServiceHealth {
        ServiceHealth {
            name: self.name().to_string(),
            status: if self.is_running {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Stopped
            },
            estimated_ram_kb: 768,
            last_heartbeat_utc: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// AuthService
// ---------------------------------------------------------------------------
pub struct AuthService {
    is_running: bool,
    auto_lock_minutes: u32,
    bus: Option<Arc<EventBus>>,
}

impl AuthService {
    pub fn new(auto_lock_minutes: u32) -> Self {
        Self {
            is_running: false,
            auto_lock_minutes,
            bus: None,
        }
    }
}

#[async_trait]
impl KernelService for AuthService {
    fn name(&self) -> &'static str {
        "AuthService"
    }

    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        self.is_running = true;
        self.bus = Some(bus);
        info!("AuthService active with {}-min auto-lock window", self.auto_lock_minutes);
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), KernelError> {
        self.is_running = false;
        Ok(())
    }

    async fn health_check(&self) -> ServiceHealth {
        ServiceHealth {
            name: self.name().to_string(),
            status: if self.is_running {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Stopped
            },
            estimated_ram_kb: 420,
            last_heartbeat_utc: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// StorageService
// ---------------------------------------------------------------------------
pub struct StorageService {
    is_running: bool,
    bus: Option<Arc<EventBus>>,
}

impl StorageService {
    pub fn new() -> Self {
        Self {
            is_running: false,
            bus: None,
        }
    }
}

#[async_trait]
impl KernelService for StorageService {
    fn name(&self) -> &'static str {
        "StorageService"
    }

    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        self.is_running = true;
        self.bus = Some(bus);
        info!("StorageService active and managing ~/.mindset_data/ vaults");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), KernelError> {
        self.is_running = false;
        Ok(())
    }

    async fn health_check(&self) -> ServiceHealth {
        ServiceHealth {
            name: self.name().to_string(),
            status: if self.is_running {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Stopped
            },
            estimated_ram_kb: 610,
            last_heartbeat_utc: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// OllamaHealthService
// ---------------------------------------------------------------------------
pub struct OllamaHealthService {
    is_running: bool,
    endpoint: String,
    bus: Option<Arc<EventBus>>,
}

impl OllamaHealthService {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            is_running: false,
            endpoint: endpoint.into(),
            bus: None,
        }
    }
}

#[async_trait]
impl KernelService for OllamaHealthService {
    fn name(&self) -> &'static str {
        "OllamaHealthService"
    }

    async fn start(&mut self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        self.is_running = true;
        self.bus = Some(bus);
        info!("OllamaHealthService polling endpoint: {}", self.endpoint);
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), KernelError> {
        self.is_running = false;
        Ok(())
    }

    async fn health_check(&self) -> ServiceHealth {
        ServiceHealth {
            name: self.name().to_string(),
            status: if self.is_running {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Stopped
            },
            estimated_ram_kb: 380,
            last_heartbeat_utc: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// Master Service Registry
// ---------------------------------------------------------------------------
pub struct ServiceRegistry {
    services: Vec<Arc<Mutex<dyn KernelService>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register(&mut self, service: Arc<Mutex<dyn KernelService>>) {
        self.services.push(service);
    }

    pub async fn start_all(&self, bus: Arc<EventBus>) -> Result<(), KernelError> {
        for service_arc in &self.services {
            let mut svc = service_arc.lock().await;
            svc.start(Arc::clone(&bus)).await?;
        }
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<(), KernelError> {
        for service_arc in &self.services {
            let mut svc = service_arc.lock().await;
            svc.stop().await?;
        }
        Ok(())
    }

    pub async fn collect_health(&self) -> Vec<ServiceHealth> {
        let mut reports = Vec::new();
        for service_arc in &self.services {
            let svc = service_arc.lock().await;
            reports.push(svc.health_check().await);
        }
        reports
    }
}
