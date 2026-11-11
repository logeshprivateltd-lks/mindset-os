//! MindSet Authentication & Security IPC Command Handlers
//! Multi-modal biometrics, Smart Lock control, Privacy Blur, and UAC elevation
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::auth::{
    AuthSession, AuthStatus, BiometricMatchResult, FaceUnlockEngine, FingerprintResult,
};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn authenticate_with_pin(
    pin: String,
    state: State<'_, AppState>,
) -> Result<AuthSession, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    state
        .auth
        .unlock_with_pin(&lock, "default_student", &pin)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn authenticate_with_password(
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthSession, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    state
        .auth
        .unlock_with_password(&lock, "default_student", &password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn authenticate_with_face(
    embedding: Option<Vec<f32>>,
    state: State<'_, AppState>,
) -> Result<BiometricMatchResult, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    let emb = embedding.unwrap_or_default();
    state
        .auth
        .unlock_with_face(&lock, "default_student", &emb)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn authenticate_with_fingerprint(
    state: State<'_, AppState>,
) -> Result<FingerprintResult, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    state
        .auth
        .unlock_with_fingerprint(&lock, "default_student")
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn enroll_face(
    embedding: Vec<f32>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    FaceUnlockEngine::enroll_face(&lock, "default_student", &embedding)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn trigger_lock_screen(state: State<'_, AppState>) -> Result<bool, String> {
    state.auth.lock();
    Ok(true)
}

#[tauri::command]
pub async fn trigger_privacy_blur(
    blurred: Option<bool>,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    match blurred {
        Some(b) => {
            state.auth.set_privacy_blur(b);
            Ok(b)
        }
        None => Ok(state.auth.toggle_privacy_blur()),
    }
}

#[tauri::command]
pub async fn request_uac_elevation(
    pin_or_password: String,
    auth_type: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let conn = state.db.get_core_connection();
    let lock = conn.lock().map_err(|e| e.to_string())?;
    let is_pin = auth_type.to_lowercase() == "pin";
    state
        .auth
        .request_uac_elevation(&lock, "default_student", &pin_or_password, is_pin)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_uac_elevated(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.auth.is_uac_elevated())
}

#[tauri::command]
pub async fn is_system_locked(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.auth.is_locked())
}

#[tauri::command]
pub async fn is_privacy_blurred(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.auth.is_privacy_blurred())
}

#[tauri::command]
pub async fn get_auth_status(state: State<'_, AppState>) -> Result<AuthStatus, String> {
    Ok(state.auth.get_status())
}
