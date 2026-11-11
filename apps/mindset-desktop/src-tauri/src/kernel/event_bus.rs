//! MindSet Kernel Typed Event Bus
//! Thread-safe broadcast bus with automatic webview forwarding & circular history
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;
use tracing::{debug, warn};

pub const MAX_EVENT_HISTORY: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    pub has_mic: bool,
    pub has_webcam: bool,
    pub has_fingerprint: bool,
    pub has_bluetooth: bool,
    pub has_wifi_monitor: bool,
    pub wifi_adapter_name: Option<String>,
    pub is_external_wifi: bool,
    pub detected_at_utc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum KernelEvent {
    SystemReady {
        uptime_ms: u64,
        hardware: HardwareCapabilities,
    },
    ServiceStateChanged {
        service_name: String,
        status: String,
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
    EmergencyRamPurgeExecuted {
        reclaimed_bytes: u64,
        os_shell_rss_mb: f64,
    },
    HardwareStateChanged {
        capabilities: HardwareCapabilities,
    },
    SecurityLockTriggered {
        reason: String,
    },
    SyncOplogAppended {
        op_id: String,
        row_id: String,
        table_name: String,
    },
}

pub struct EventBus {
    sender: broadcast::Sender<KernelEvent>,
    app_handle: RwLock<Option<AppHandle>>,
    history: Mutex<VecDeque<KernelEvent>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Arc<Self> {
        let (sender, _) = broadcast::channel(capacity);
        Arc::new(Self {
            sender,
            app_handle: RwLock::new(None),
            history: Mutex::new(VecDeque::with_capacity(MAX_EVENT_HISTORY)),
        })
    }

    pub fn with_app_handle(capacity: usize, app_handle: AppHandle) -> Arc<Self> {
        let (sender, _) = broadcast::channel(capacity);
        Arc::new(Self {
            sender,
            app_handle: RwLock::new(Some(app_handle)),
            history: Mutex::new(VecDeque::with_capacity(MAX_EVENT_HISTORY)),
        })
    }

    pub fn set_app_handle(&self, handle: AppHandle) {
        if let Ok(mut lock) = self.app_handle.write() {
            *lock = Some(handle);
        }
    }

    pub fn publish(&self, event: KernelEvent) {
        debug!("Publishing kernel event: {:?}", event);

        // 1. Record into circular memory history
        if let Ok(mut hist) = self.history.lock() {
            if hist.len() >= MAX_EVENT_HISTORY {
                hist.pop_front();
            }
            hist.push_back(event.clone());
        }

        // 2. Forward to webview frontend if Tauri app handle is available
        if let Ok(lock) = self.app_handle.read() {
            if let Some(ref handle) = *lock {
                if let Err(e) = handle.emit("mindset://kernel-event", &event) {
                    warn!("Failed to forward kernel event to frontend: {:?}", e);
                }
            }
        }

        // 3. Broadcast to native Tokio subscribers
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<KernelEvent> {
        self.sender.subscribe()
    }

    pub fn get_history(&self, limit: usize) -> Vec<KernelEvent> {
        let effective_limit = limit.min(MAX_EVENT_HISTORY);
        if let Ok(hist) = self.history.lock() {
            hist.iter()
                .rev()
                .take(effective_limit)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_circular_history_cap() {
        let bus = EventBus::new(256);

        // Publish 250 events into a 200-capacity ring buffer
        for i in 0..250 {
            bus.publish(KernelEvent::TaskCompleted {
                task_id: format!("task-{}", i),
                xp_earned: i as u32,
            });
        }

        let history_50 = bus.get_history(50);
        assert_eq!(history_50.len(), 50);

        let history_max = bus.get_history(300);
        assert_eq!(history_max.len(), MAX_EVENT_HISTORY);

        // The oldest remaining event should be task-50, newest should be task-249
        if let KernelEvent::TaskCompleted { task_id, .. } = &history_max[0] {
            assert_eq!(task_id, "task-50");
        } else {
            panic!("Expected TaskCompleted event");
        }

        if let KernelEvent::TaskCompleted { task_id, .. } = &history_max[MAX_EVENT_HISTORY - 1] {
            assert_eq!(task_id, "task-249");
        } else {
            panic!("Expected TaskCompleted event");
        }
    }
}
