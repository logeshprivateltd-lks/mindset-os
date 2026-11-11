//! MindSet OS — AI Subsystem & Laya Tauri Command Handlers
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod laya_engine;

use laya_engine::{
    LayaEngine, LayaStreamRequest, LocalModelInfo,
};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{command, AppHandle, State};

pub struct ActiveModelState {
    pub current_model: Mutex<String>,
    pub is_streaming: Arc<AtomicBool>,
}

impl ActiveModelState {
    pub fn new() -> Self {
        Self {
            current_model: Mutex::new("gemma2:9b-instruct-q4_K_M".to_string()),
            is_streaming: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[command]
pub async fn ai_chat_stream(
    app_handle: AppHandle,
    req: LayaStreamRequest,
    state: State<'_, ActiveModelState>,
) -> Result<String, String> {
    LayaEngine::stream_chat(
        &app_handle,
        req,
        None,
        Some(Arc::clone(&state.is_streaming)),
    )
    .await
}

#[command]
pub async fn ai_generate_embedding(
    model: String,
    text: String,
) -> Result<Vec<f32>, String> {
    LayaEngine::generate_embedding(&model, &text, None).await
}

#[command]
pub async fn ai_list_local_models() -> Result<Vec<LocalModelInfo>, String> {
    LayaEngine::list_local_models(None).await
}

#[command]
pub async fn ai_set_active_model(
    model_name: String,
    state: State<'_, ActiveModelState>,
) -> Result<String, String> {
    let mut lock = state
        .current_model
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    *lock = model_name.clone();
    Ok(model_name)
}

#[command]
pub async fn ai_get_active_model(
    state: State<'_, ActiveModelState>,
) -> Result<String, String> {
    let lock = state
        .current_model
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    Ok(lock.clone())
}

#[command]
pub async fn ai_unload_model(model_name: String) -> Result<bool, String> {
    LayaEngine::unload_active_model_from_ram(&model_name, None).await
}

#[command]
pub async fn ai_restart_ollama() -> Result<bool, String> {
    // Probing Ollama tags endpoint as health ping
    match LayaEngine::list_local_models(None).await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Ollama ping failed: {}", e)),
    }
}

#[command]
pub async fn ai_execute_structured_tool_call(
    model: String,
    system_prompt: String,
    user_prompt: String,
    json_schema: Option<String>,
) -> Result<serde_json::Value, String> {
    LayaEngine::execute_structured_tool_call(
        &model,
        &system_prompt,
        &user_prompt,
        json_schema.as_deref(),
        None,
    )
    .await
}
