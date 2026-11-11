//! MindSet Focus Guard — System-Level Distraction Interceptor & Process Monitor
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use focus_policy_engine::{PolicyConfiguration, PolicyEvaluator};
use std::time::Duration;
use sysinfo::System;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting MindSet Focus Guard daemon [LKS PVT LTD]...");

    let config = PolicyConfiguration::default();
    let evaluator = PolicyEvaluator::new(config);
    let mut sys = System::new_all();

    let mut interval = tokio::time::interval(Duration::from_secs(3));
    let mut scan_count: u64 = 0;

    loop {
        interval.tick().await;
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        scan_count = scan_count.wrapping_add(1);

        for (pid, process) in sys.processes() {
            let proc_name = process.name().to_string_lossy();
            if let Some(rule) = evaluator.check_process(&proc_name) {
                warn!(
                    "Distraction detected! PID: {}, Process: {}, Rule: {}, Action: {:?}",
                    pid, proc_name, rule.id, rule.action
                );
            }
        }

        if scan_count % 20 == 0 {
            info!("Focus Guard heartbeat ok ({} processes inspected)", sys.processes().len());
        }
    }
}

