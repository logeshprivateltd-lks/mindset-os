//! MindSet OS — AI Subsystem & Laya Engine Integration
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod laya_engine;
pub mod screen_reader;

pub use laya_engine::*;
pub use screen_reader::*;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ActiveModelState {
    pub current_model: Mutex<String>,
    pub is_streaming: Arc<AtomicBool>,
}

impl Default for ActiveModelState {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveModelState {
    pub fn new() -> Self {
        Self {
            current_model: Mutex::new("gemma2:2b".to_string()),
            is_streaming: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_active_model(&self, model_name: String) -> Result<String, String> {
        let mut lock = self
            .current_model
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        *lock = model_name.clone();
        Ok(model_name)
    }

    pub fn get_active_model(&self) -> Result<String, String> {
        let lock = self
            .current_model
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        Ok(lock.clone())
    }
}
