//! MindSet Kernel Typed Event Bus
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;
use tracing::{debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    pub has_mic: bool,
    pub has_webcam: bool,
    pub has_fingerprint: bool,
    pub has_bluetooth: bool,
    pub has_wifi_monitor: bool,
    pub detected_at_utc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum KernelEvent {
    SystemReady {
        uptime_ms: u64,
        hardware: HardwareCapabilities,
    },
    FocusSessionStateChanged {
        session_id: String,
        state: String,
        elapsed_secs: u32,
    },
    TaskCompleted {
        task_id: String,
        xp_earned: u32,
    },
    MoodLogged {
        mood: String,
        energy_tier: String,
    },
    FreeTimeWindowDetected {
        start_time: String,
        duration_minutes: u32,
    },
    RamThresholdWarning {
        process_rss_mb: u64,
        system_used_percent: f32,
    },
    HardwareStateChanged {
        capabilities: HardwareCapabilities,
    },
    SecurityLockTriggered {
        reason: String,
    },
    ProfileUpdated {
        user_id: String,
    },
    SyncOplogAppended {
        op_id: String,
        row_id: String,
        table_name: String,
    },
}

pub struct EventBus {
    sender: broadcast::Sender<KernelEvent>,
    app_handle: Option<AppHandle>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Arc<Self> {
        let (sender, _) = broadcast::channel(capacity);
        Arc::new(Self {
            sender,
            app_handle: None,
        })
    }

    pub fn with_app_handle(capacity: usize, app_handle: AppHandle) -> Arc<Self> {
        let (sender, _) = broadcast::channel(capacity);
        Arc::new(Self {
            sender,
            app_handle: Some(app_handle),
        })
    }

    pub fn publish(&self, event: KernelEvent) {
        debug!("Publishing kernel event: {:?}", event);

        // Forward to webview frontend if Tauri app handle is available
        if let Some(ref handle) = self.app_handle {
            if let Err(e) = handle.emit("mindset://kernel-event", &event) {
                warn!("Failed to forward kernel event to frontend: {:?}", e);
            }
        }

        // Broadcast to native Tokio subscribers
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<KernelEvent> {
        self.sender.subscribe()
    }
}
