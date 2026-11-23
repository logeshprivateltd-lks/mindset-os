//! MindSet Connect — Desktop Module 5: Communication & Wireless Hardware Bridge (`hardware_bridge.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.
//!
//! STRICT RULE 2: Automatically strip or skip passwords, OTP codes, and authentication
//! tokens when syncing the universal clipboard across devices.

use chrono::Utc;
use connect_protocol::{
    ClipboardSyncPayload, MediaControlCommand, MediaPlaybackAction, NotificationMirrorItem,
    PresentationClickerCommand, ScannedDocumentPayload, SeminarClickerAction, StreamState,
    WirelessMicrophoneStatus, WirelessWebcamStatus,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use uuid::Uuid;

pub struct DesktopHardwareBridge {
    webcam_status: Mutex<WirelessWebcamStatus>,
    mic_status: Mutex<WirelessMicrophoneStatus>,
    data_dir: PathBuf,
}

impl DesktopHardwareBridge {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home.join(".mindset_data");
        let _ = fs::create_dir_all(data_dir.join("vault").join("scanned_notes"));

        Self {
            webcam_status: Mutex::new(WirelessWebcamStatus {
                is_active: true,
                state: StreamState::Streaming,
                streaming_device_id: Some("dev-phone-galaxy-s24".to_string()),
                streaming_device_name: Some("Galaxy S24 (Main Sensor)".to_string()),
                resolution: "1080p (1920x1080)".to_string(),
                fps: 30,
                latency_ms: 18,
                battery_percent: Some(82),
                use_front_camera: false,
                stream_url_or_endpoint: Some("ws://192.168.1.112:52432/live/camera".to_string()),
            }),
            mic_status: Mutex::new(WirelessMicrophoneStatus {
                is_active: true,
                state: StreamState::Streaming,
                streaming_device_id: Some("dev-phone-galaxy-s24".to_string()),
                streaming_device_name: Some("Galaxy S24 (Dual Mic Array)".to_string()),
                sample_rate_hz: 16000,
                latency_ms: 12,
                noise_suppression: true,
                audio_level_db: -24.5,
            }),
            data_dir,
        }
    }

    /// Process clipboard sync with strict privacy filter (Rule 2)
    pub fn filter_and_sync_clipboard(
        &self,
        content: &str,
        source_device_id: &str,
        source_device_name: &str,
    ) -> ClipboardSyncPayload {
        let trimmed = content.trim();
        let mut is_sensitive = false;
        let mut filter_reason = None;
        let mut filtered_content = content.to_string();

        // 1. Check for pure OTP patterns (4 to 8 consecutive digits)
        let is_otp =
            trimmed.len() >= 4 && trimmed.len() <= 8 && trimmed.chars().all(|c| c.is_ascii_digit());
        if is_otp {
            is_sensitive = true;
            filter_reason = Some(
                "One-Time Password (OTP) detected and blocked from cross-device sync".to_string(),
            );
            filtered_content = "[SENSITIVE OTP BLOCKED]".to_string();
        }

        // 2. Check for keywords indicating credentials / bearer tokens / keys
        let lower = content.to_lowercase();
        if lower.contains("bearer ")
            || lower.contains("apikey")
            || lower.contains("api_key")
            || lower.contains("secret")
            || lower.contains("password=")
            || lower.contains("passwd=")
        {
            is_sensitive = true;
            filter_reason =
                Some("Authentication token or secret password detected and stripped".to_string());
            filtered_content = "[CREDENTIAL STRIPPED BY MINDSET CONNECT PRIVACY GUARD]".to_string();
        }

        ClipboardSyncPayload {
            payload_id: format!("clp-{}", Uuid::new_v4().simple()),
            source_device_id: source_device_id.to_string(),
            source_device_name: source_device_name.to_string(),
            content: filtered_content,
            is_sensitive_filtered: is_sensitive,
            filter_reason,
            timestamp_utc: Utc::now(),
        }
    }

    /// Handle seminar clicker action
    pub fn handle_clicker_action(
        &self,
        action: SeminarClickerAction,
    ) -> PresentationClickerCommand {
        PresentationClickerCommand {
            command_id: format!("clk-{}", Uuid::new_v4().simple()),
            action,
            laser_x_percent: None,
            laser_y_percent: None,
            timestamp_utc: Utc::now(),
        }
    }

    /// Handle media control
    #[allow(dead_code)]
    pub fn handle_media_action(
        &self,
        action: MediaPlaybackAction,
        volume: Option<u8>,
    ) -> MediaControlCommand {
        MediaControlCommand {
            action,
            device_id: "mindset-host-laptop-01".to_string(),
            volume_level: volume,
        }
    }

    /// Get current webcam stream status
    pub fn get_webcam_status(&self) -> WirelessWebcamStatus {
        self.webcam_status.lock().unwrap().clone()
    }

    /// Toggle or configure wireless webcam stream
    pub fn toggle_webcam(
        &self,
        active: bool,
        resolution: &str,
        use_front: bool,
    ) -> WirelessWebcamStatus {
        let mut cam = self.webcam_status.lock().unwrap();
        cam.is_active = active;
        cam.state = if active {
            StreamState::Streaming
        } else {
            StreamState::Disconnected
        };
        cam.resolution = resolution.to_string();
        cam.use_front_camera = use_front;
        cam.clone()
    }

    /// Get current mic stream status
    pub fn get_mic_status(&self) -> WirelessMicrophoneStatus {
        self.mic_status.lock().unwrap().clone()
    }

    /// Toggle wireless mic stream
    pub fn toggle_mic(&self, active: bool, noise_suppression: bool) -> WirelessMicrophoneStatus {
        let mut mic = self.mic_status.lock().unwrap();
        mic.is_active = active;
        mic.state = if active {
            StreamState::Streaming
        } else {
            StreamState::Disconnected
        };
        mic.noise_suppression = noise_suppression;
        mic.clone()
    }

    /// Ingest scanned document from mobile camera into laptop vault & RAG pipeline
    pub fn ingest_scanned_document(
        &self,
        payload: ScannedDocumentPayload,
    ) -> Result<String, String> {
        let scans_dir = self.data_dir.join("vault").join("scanned_notes");
        let safe_title = payload.document_title.replace(' ', "_").replace('/', "_");
        let file_name = format!("{}_{}.pdf", safe_title, payload.scan_id);
        let dest_path = scans_dir.join(&file_name);

        // In real execution, writes PDF bundle; create placeholder file
        let summary_text = format!(
            "# MindSet Scanned Document\nTitle: {}\nSubject: {}\nPages: {}\nScanned via: {}\nTimestamp: {}\n",
            payload.document_title,
            payload.subject_tag,
            payload.total_pages,
            payload.source_device_id,
            payload.timestamp_utc
        );
        fs::write(&dest_path, summary_text).map_err(|e| e.to_string())?;

        Ok(dest_path.to_string_lossy().to_string())
    }

    /// List sample mirrored notifications
    #[allow(dead_code)]
    pub fn get_mirrored_notifications(&self) -> Vec<NotificationMirrorItem> {
        vec![
            NotificationMirrorItem {
                notification_id: "notif-01".to_string(),
                source_device_id: "dev-phone-galaxy-s24".to_string(),
                app_name: "Phone".to_string(),
                title: "Incoming Call: Mom".to_string(),
                body: "Urgent contact - Focus Guard auto-allowed during study session".to_string(),
                category: "incoming_call".to_string(),
                is_actionable: true,
                timestamp_utc: Utc::now() - chrono::Duration::minutes(2),
            },
            NotificationMirrorItem {
                notification_id: "notif-02".to_string(),
                source_device_id: "dev-phone-galaxy-s24".to_string(),
                app_name: "School Study Group".to_string(),
                title: "Physics Teacher uploaded Formula Sheet".to_string(),
                body: "Ray Optics Chapter 4 assignment due tomorrow 9 AM".to_string(),
                category: "study_reminder".to_string(),
                is_actionable: true,
                timestamp_utc: Utc::now() - chrono::Duration::minutes(15),
            },
        ]
    }
}
