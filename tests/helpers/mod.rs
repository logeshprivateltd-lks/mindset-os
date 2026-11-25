//! Test Harness & Helper Utilities for MindSet OS E2E Suite

pub mod headless_harness;
pub mod mock_data;

pub use headless_harness::{HeadlessHarness, AUTH_LOCK};
pub use mock_data::*;
