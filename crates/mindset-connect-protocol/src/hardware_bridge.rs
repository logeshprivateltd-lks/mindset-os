//! MindSet Connect — Module 5: Communication & Wireless Hardware Bridge (`hardware_bridge.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Sensitive-filtered clipboard item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardSyncPayload {
    pub payload_id: String,
    pub source_device_id: String,
    pub source_device_name: String,
    pub content: String,
    pub is_sensitive_filtered: bool,
    pub filter_reason: Option<String>,
    pub timestamp_utc: DateTime<Utc>,
}

/// Notification mirrored from phone to laptop or laptop to phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMirrorItem {
    pub notification_id: String,
    pub source_device_id: String,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub category: String, // "urgent_message", "incoming_call", "study_reminder", "system"
    pub is_actionable: bool,
    pub timestamp_utc: DateTime<Utc>,
}

/// Classroom seminar / presentation clicker command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeminarClickerAction {
    NextSlide,
    PrevSlide,
    ToggleLaser,
    BlankScreen,
    StartPresentation,
    StopPresentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationClickerCommand {
    pub command_id: String,
    pub action: SeminarClickerAction,
    pub laser_x_percent: Option<f32>,
    pub laser_y_percent: Option<f32>,
    pub timestamp_utc: DateTime<Utc>,
}

/// Quick media control command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaPlaybackAction {
    PlayPause,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
    MuteToggle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaControlCommand {
    pub action: MediaPlaybackAction,
    pub device_id: String,
    pub volume_level: Option<u8>,
}

/// Hardware bridge operating state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamState {
    Disconnected,
    Connecting,
    Streaming,
    Paused,
    Error,
}

/// Wireless Webcam bridge status & telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessWebcamStatus {
    pub is_active: bool,
    pub state: StreamState,
    pub streaming_device_id: Option<String>,
    pub streaming_device_name: Option<String>,
    pub resolution: String, // "1080p (1920x1080)", "720p (1280x720)"
    pub fps: u8,
    pub latency_ms: u32,
    pub battery_percent: Option<u8>,
    pub use_front_camera: bool,
    pub stream_url_or_endpoint: Option<String>,
}

impl Default for WirelessWebcamStatus {
    fn default() -> Self {
        Self {
            is_active: false,
            state: StreamState::Disconnected,
            streaming_device_id: None,
            streaming_device_name: None,
            resolution: "1080p (1920x1080)".to_string(),
            fps: 30,
            latency_ms: 0,
            battery_percent: None,
            use_front_camera: false,
            stream_url_or_endpoint: None,
        }
    }
}

/// Wireless Microphone bridge status (for Whisper voice typing & speech AI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessMicrophoneStatus {
    pub is_active: bool,
    pub state: StreamState,
    pub streaming_device_id: Option<String>,
    pub streaming_device_name: Option<String>,
    pub sample_rate_hz: u32, // 16000 or 44100
    pub latency_ms: u32,
    pub noise_suppression: bool,
    pub audio_level_db: f32,
}

impl Default for WirelessMicrophoneStatus {
    fn default() -> Self {
        Self {
            is_active: false,
            state: StreamState::Disconnected,
            streaming_device_id: None,
            streaming_device_name: None,
            sample_rate_hz: 16000,
            latency_ms: 0,
            noise_suppression: true,
            audio_level_db: -60.0,
        }
    }
}

/// Document Scanner capture payload beamed from phone to laptop RAG pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedDocumentPayload {
    pub scan_id: String,
    pub source_device_id: String,
    pub document_title: String,
    pub subject_tag: String, // e.g. "Physics", "Chemistry", "Mathematics", "General"
    pub total_pages: u32,
    pub images_base64: Vec<String>,
    pub auto_deskewed: bool,
    pub contrast_boosted: bool,
    pub target_vault_path: String,
    pub trigger_rag_ingestion: bool,
    pub timestamp_utc: DateTime<Utc>,
}
