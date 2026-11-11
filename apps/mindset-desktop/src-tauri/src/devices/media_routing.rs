//! MindSet OS — Audio/Camera Hardware Fallback Router
//! Automatic graceful routing to MindSet Connect Wireless Phone Bridge
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDevice {
    pub id: String,
    pub name: String,
    pub kind: String, // "audio_input" | "audio_output" | "video_input"
    pub is_default: bool,
    pub is_fallback_virtual: bool,
    pub status: String, // "Active" | "Ready" | "Bridged via MindSet Connect" | "Fallback Active"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaRoutingReport {
    pub audio_inputs: Vec<MediaDevice>,
    pub audio_outputs: Vec<MediaDevice>,
    pub video_inputs: Vec<MediaDevice>,
    pub active_audio_input_id: String,
    pub active_audio_output_id: String,
    pub active_video_input_id: String,
    pub mic_fallback_active: bool,
    pub mic_fallback_label: String,
    pub camera_fallback_active: bool,
    pub camera_fallback_label: String,
    pub bluetooth_fallback_active: bool,
    pub bluetooth_fallback_label: String,
    pub timestamp_utc: String,
}

struct MediaRoutingState {
    active_mic_id: String,
    active_output_id: String,
    active_cam_id: String,
}

static MEDIA_STATE: OnceLock<RwLock<MediaRoutingState>> = OnceLock::new();

fn get_state() -> &'static RwLock<MediaRoutingState> {
    MEDIA_STATE.get_or_init(|| {
        RwLock::new(MediaRoutingState {
            active_mic_id: "mic-phone-bridge".to_string(),
            active_output_id: "out-speakers-default".to_string(),
            active_cam_id: "cam-phone-bridge".to_string(),
        })
    })
}

pub struct MediaRoutingManager;

impl MediaRoutingManager {
    /// Get current audio, video, and peripheral routing with fallback indicators
    pub async fn get_routing_status(has_physical_mic: bool, has_physical_cam: bool, has_bluetooth: bool) -> MediaRoutingReport {
        let state_lock = get_state().read().unwrap();

        // 1. Audio Inputs (Microphones)
        let mut audio_inputs = Vec::new();
        if has_physical_mic {
            audio_inputs.push(MediaDevice {
                id: "mic-internal".to_string(),
                name: "Realtek High Definition Array Microphone".to_string(),
                kind: "audio_input".to_string(),
                is_default: state_lock.active_mic_id == "mic-internal",
                is_fallback_virtual: false,
                status: "Ready".to_string(),
            });
        }

        // MindSet Connect Phone Bridge (always available as wireless fallback)
        audio_inputs.push(MediaDevice {
            id: "mic-phone-bridge".to_string(),
            name: "MindSet Connect Wireless Phone Mic (16kHz Opus Stream)".to_string(),
            kind: "audio_input".to_string(),
            is_default: state_lock.active_mic_id == "mic-phone-bridge" || !has_physical_mic,
            is_fallback_virtual: true,
            status: "Bridged via MindSet Connect".to_string(),
        });

        // Text-Input Fallback (Whisper typing fallback)
        audio_inputs.push(MediaDevice {
            id: "mic-text-input".to_string(),
            name: "Virtual Keyboard & Text-Input Fallback".to_string(),
            kind: "audio_input".to_string(),
            is_default: state_lock.active_mic_id == "mic-text-input",
            is_fallback_virtual: true,
            status: "Fallback Active".to_string(),
        });

        // 2. Audio Outputs (Speakers / Headphones)
        let audio_outputs = vec![
            MediaDevice {
                id: "out-speakers-default".to_string(),
                name: "Realtek High Definition Audio Speakers".to_string(),
                kind: "audio_output".to_string(),
                is_default: state_lock.active_output_id == "out-speakers-default",
                is_fallback_virtual: false,
                status: "Active".to_string(),
            },
            MediaDevice {
                id: "out-headphones-jack".to_string(),
                name: "3.5mm Stereo Headphone Output".to_string(),
                kind: "audio_output".to_string(),
                is_default: state_lock.active_output_id == "out-headphones-jack",
                is_fallback_virtual: false,
                status: "Ready".to_string(),
            },
        ];

        // 3. Video Inputs (Webcams / Visual capture)
        let mut video_inputs = Vec::new();
        if has_physical_cam {
            video_inputs.push(MediaDevice {
                id: "cam-internal-720p".to_string(),
                name: "Integrated HD Webcam (720p / 30fps)".to_string(),
                kind: "video_input".to_string(),
                is_default: state_lock.active_cam_id == "cam-internal-720p",
                is_fallback_virtual: false,
                status: "Ready".to_string(),
            });
        }

        // MindSet Connect Phone Camera Stream
        video_inputs.push(MediaDevice {
            id: "cam-phone-bridge".to_string(),
            name: "MindSet Connect 4K/1080p Mobile Camera Stream".to_string(),
            kind: "video_input".to_string(),
            is_default: state_lock.active_cam_id == "cam-phone-bridge" || !has_physical_cam,
            is_fallback_virtual: true,
            status: "Bridged via MindSet Connect".to_string(),
        });

        // Virtual Avatar / Screen Region Solver
        video_inputs.push(MediaDevice {
            id: "cam-screen-region".to_string(),
            name: "MindSet Screen Doubt Solver Virtual Input (Alt + S)".to_string(),
            kind: "video_input".to_string(),
            is_default: state_lock.active_cam_id == "cam-screen-region",
            is_fallback_virtual: true,
            status: "Ready".to_string(),
        });

        let mic_fallback_active = !has_physical_mic || state_lock.active_mic_id == "mic-phone-bridge";
        let mic_fallback_label = if !has_physical_mic {
            "Routed to MindSet Connect Wireless Phone Bridge".to_string()
        } else {
            "Internal Microphone Active".to_string()
        };

        let camera_fallback_active = !has_physical_cam || state_lock.active_cam_id == "cam-phone-bridge";
        let camera_fallback_label = if !has_physical_cam {
            "Routed to MindSet Connect Phone Camera Stream".to_string()
        } else {
            "Integrated HD Webcam Active".to_string()
        };

        let bluetooth_fallback_active = !has_bluetooth;
        let bluetooth_fallback_label = if !has_bluetooth {
            "Routed to Local Wi-Fi Direct Bridge (Zero Bluetooth Hardware Required)".to_string()
        } else {
            "Bluetooth 5.2 Controller Active".to_string()
        };

        MediaRoutingReport {
            active_audio_input_id: state_lock.active_mic_id.clone(),
            active_audio_output_id: state_lock.active_output_id.clone(),
            active_video_input_id: state_lock.active_cam_id.clone(),
            audio_inputs,
            audio_outputs,
            video_inputs,
            mic_fallback_active,
            mic_fallback_label,
            camera_fallback_active,
            camera_fallback_label,
            bluetooth_fallback_active,
            bluetooth_fallback_label,
            timestamp_utc: Utc::now().to_rfc3339(),
        }
    }

    /// Select active media device
    pub async fn select_active_device(kind: &str, device_id: &str) -> Result<(), String> {
        let mut state_lock = get_state().write().map_err(|e| e.to_string())?;
        match kind {
            "audio_input" => {
                state_lock.active_mic_id = device_id.to_string();
                info!("Switched default audio input to: {}", device_id);
            }
            "audio_output" => {
                state_lock.active_output_id = device_id.to_string();
                info!("Switched default audio output to: {}", device_id);
            }
            "video_input" => {
                state_lock.active_cam_id = device_id.to_string();
                info!("Switched default video source to: {}", device_id);
            }
            _ => return Err(format!("Unknown media device kind: {}", kind)),
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_media_routing_fallbacks() {
        // Test scenario: No physical microphone, no physical webcam, no bluetooth
        let report = MediaRoutingManager::get_routing_status(false, false, false).await;

        assert!(report.mic_fallback_active, "Mic fallback must be active when physical mic missing");
        assert_eq!(report.mic_fallback_label, "Routed to MindSet Connect Wireless Phone Bridge");

        assert!(report.camera_fallback_active, "Camera fallback must be active when physical cam missing");
        assert_eq!(report.camera_fallback_label, "Routed to MindSet Connect Phone Camera Stream");

        assert!(report.bluetooth_fallback_active, "Bluetooth fallback must be active");
        assert!(report.bluetooth_fallback_label.contains("Local Wi-Fi Direct Bridge"));

        assert!(!report.audio_inputs.is_empty());
        assert!(!report.video_inputs.is_empty());
        assert!(!report.audio_outputs.is_empty());
    }

    #[tokio::test]
    async fn test_device_selection() {
        let res = MediaRoutingManager::select_active_device("audio_input", "mic-text-input").await;
        assert!(res.is_ok());
    }
}
