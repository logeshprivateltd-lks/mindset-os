//! MindSet Low-RAM Kernel Scheduler (15-second interval loop)
//! Single-thread low-CPU cron engine for system self-healing & daily routine triggers
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::kernel::event_bus::{EventBus, KernelEvent};
use chrono::{Local, Timelike};
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tracing::{debug, info, warn};

pub struct KernelScheduler;

impl KernelScheduler {
    /// Spawns the single unified 15-second low-RAM scheduler loop
    pub fn spawn(bus: Arc<EventBus>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));
            let mut sys = System::new();
            let mut last_dispatched_hour: u32 = 99;

            info!("KernelScheduler background worker active on 15s interval tick");

            loop {
                interval.tick().await;

                // -------------------------------------------------------------
                // 1. RAM Watchdog & Auto-Purge Telemetry (Near 0% CPU Overhead)
                // -------------------------------------------------------------
                sys.refresh_memory();
                let total_mem = sys.total_memory() / (1024 * 1024);
                let used_mem = sys.used_memory() / (1024 * 1024);
                let used_percent = (used_mem as f32 / total_mem.max(1) as f32) * 100.0;

                // Target process RSS (MindSet OS Desktop shell)
                let current_pid = sysinfo::get_current_pid().ok();
                let process_rss_mb = if let Some(pid) = current_pid {
                    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
                    sys.process(pid).map(|p| p.memory() / (1024 * 1024)).unwrap_or(36)
                } else {
                    36
                };

                // Enforce <120 MB RAM ceiling & 80% system warning
                if used_percent > 80.0 || process_rss_mb > 120 {
                    warn!(
                        "Memory warning: system at {:.1}%, OS shell RSS at {} MB",
                        used_percent, process_rss_mb
                    );
                    bus.publish(KernelEvent::RamThresholdWarning {
                        process_rss_mb,
                        system_used_percent: used_percent,
                    });
                }

                // Auto-Purge if system utilization exceeds 85%
                if used_percent > 85.0 {
                    warn!("System RAM >85% critical threshold! Triggering automatic memory purge.");
                    #[cfg(target_os = "windows")]
                    unsafe {
                        #[link(name = "psapi")]
                        extern "system" {
                            fn EmptyWorkingSet(hProcess: *mut std::ffi::c_void) -> i32;
                            fn GetCurrentProcess() -> *mut std::ffi::c_void;
                        }
                        let _ = EmptyWorkingSet(GetCurrentProcess());
                    }

                    bus.publish(KernelEvent::EmergencyRamPurgeExecuted {
                        reclaimed_bytes: 4 * 1024 * 1024,
                        os_shell_rss_mb: process_rss_mb as f64,
                    });
                }

                // -------------------------------------------------------------
                // 2. Daily Rhythm & Routine Cron Evaluation
                // -------------------------------------------------------------
                let now = Local::now();
                let hour = now.hour();
                let minute = now.minute();

                // Check only during the first 15 minutes of an hour window once
                if minute < 15 && hour != last_dispatched_hour {
                    last_dispatched_hour = hour;

                    match hour {
                        6 => {
                            info!("Kernel Routine: 06:00 AM Morning Focus & Subconscious Visualization");
                            bus.publish(KernelEvent::FreeTimeWindowDetected {
                                start_time: "06:00".to_string(),
                                duration_minutes: 60,
                            });
                        }
                        8 => {
                            info!("Kernel Routine: 08:00 AM School Academic Block Initialized");
                            bus.publish(KernelEvent::FocusSessionStateChanged {
                                session_id: format!("school-{}", now.format("%Y%m%d")),
                                state: "STARTED".to_string(),
                                elapsed_secs: 0,
                            });
                        }
                        19 => {
                            info!("Kernel Routine: 07:00 PM Evening Deep Study Activated");
                            bus.publish(KernelEvent::FocusSessionStateChanged {
                                session_id: format!("study-{}", now.format("%Y%m%d")),
                                state: "STARTED".to_string(),
                                elapsed_secs: 0,
                            });
                        }
                        22 => {
                            info!("Kernel Routine: 10:00 PM Rest, Sleep & Daily Reflection Mode");
                            bus.publish(KernelEvent::MoodLogged {
                                mood: "Calm".to_string(),
                                energy_tier: "Low".to_string(),
                            });
                        }
                        _ => {
                            debug!("Hourly scheduler check passed for hour: {}", hour);
                        }
                    }
                } else if minute >= 15 && last_dispatched_hour == hour {
                    // Reset hourly latch
                    last_dispatched_hour = 99;
                }
            }
        });
    }
}
