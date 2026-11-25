//! MindSet OS — E2E Test Suite
//!
//! Comprehensive 4-tier test architecture covering Features 1 through 24
//! under strict headless execution, SQLite WAL isolation, and pure Rust contracts.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod helpers;
pub mod tier1_features;
pub mod tier2_boundaries;
pub mod tier3_combinatorial;
pub mod tier4_real_workloads;
