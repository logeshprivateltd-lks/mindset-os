//! MindSet OS IPC Commands Hub
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod auth_cmd;
pub mod db_cmd;
pub mod kernel_cmd;

pub use auth_cmd::*;
pub use db_cmd::*;
pub use kernel_cmd::{
    control_kernel_service, get_kernel_event_history, get_kernel_status,
    trigger_emergency_ram_purge,
};
