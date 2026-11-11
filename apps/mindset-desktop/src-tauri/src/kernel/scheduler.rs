//! MindSet Low-RAM Kernel Scheduler (15-second interval loop)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::kernel::event_bus::{EventBus, KernelEvent};
use chrono::{Local, Timelike};
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tracing::{debug, warn};

pub struct KernelScheduler;

impl KernelScheduler {
    /// Spawns the low-RAM asynchronous scheduler loop onto Tokio
    pub fn spawn(bus: Arc<EventBus>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));
            let mut sys = System::new();
            let mut last_hour_tick: u32 = 99;

            loop {
                interval.tick().await;

                // 1. Refresh minimal system memory telemetry
                sys.refresh_memory();
                let total_mem = sys.total_memory();
                let used_mem = sys.used_memory();
                let used_percent = (used_mem as f32 / total_mem.max(1) as f32) * 100.0;

                // Process RAM check (Tauri process RSS)
                let current_pid = sysinfo::get_current_pid().ok();
                let process_rss_mb = if let Some(pid) = current_pid {
                    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
                    sys.process(pid).map(|p| p.memory() / (1024 * 1024)).unwrap_or(35)
                } else {
                    35
                };

                // Watchdog Warning if overall system RAM > 85%
                if used_percent > 85.0 {
                    warn!(
                        "System RAM utilization critical: {:.1}% (Process RSS: {} MB)",
                        used_percent, process_rss_mb
                    );
                    bus.publish(KernelEvent::RamThresholdWarning {
                        process_rss_mb,
                        system_used_percent: used_percent,
                    });
                }

                // 2. Timetable & Scheduled Reflection Triggers
                let now = Local::now();
                let current_hour = now.hour();
                let current_minute = now.minute();

                // Trigger hourly check once per hour
                if current_hour != last_hour_tick {
                    last_hour_tick = current_hour;

                    if current_hour == 6 && current_minute < 15 {
                        debug!("Morning subconscious focus window triggered");
                    } else if current_hour == 22 && current_minute < 15 {
                        debug!("Evening gratitude & sleep prep window triggered");
                    }
                }
            }
        });
    }
}

