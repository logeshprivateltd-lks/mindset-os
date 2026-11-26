//! MindSet OS — Native System Subsystem (`mindset-sys`)
//! Pure-Rust OS kernel services, SQLite WAL persistence, local AI orchestrator, hardware sensors, and updater.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

#![allow(clippy::all)]

pub mod ai;
pub mod auth;
pub mod connect;
pub mod db;
pub mod devices;
pub mod fs;
pub mod kernel;
pub mod monitor;
pub mod network;
pub mod personalization;
pub mod recovery;
pub mod storage;
pub mod store;
pub mod updater;

pub use personalization::*;
pub use recovery::{check_recovery, RecoveryState, StreakState};
