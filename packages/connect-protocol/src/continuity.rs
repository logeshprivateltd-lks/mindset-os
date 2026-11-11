//! MindSet Connect — Module 4: Continuity Engine Protocol (`continuity.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of work state being handed off across devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityItemType {
    PdfTextbook,
    JournalDraft,
    ActiveTask,
    SyllabusNode,
    ScreenSolverSession,
}

/// "Continue Where You Left Off" state handoff payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityStateHandoff {
    pub session_id: String,
    pub source_device_id: String,
    pub source_device_name: String,
    pub item_type: ContinuityItemType,
    pub title: String,
    pub resource_identifier: String, // file path, task id, journal id, or syllabus topic
    pub last_read_page: Option<u32>,
    pub total_pages: Option<u32>,
    pub scroll_offset_ratio: Option<f32>,
    pub draft_content: Option<String>,
    pub active_subject: Option<String>,
    pub timestamp_utc: DateTime<Utc>,
}

/// Focus Mode profile matching Focus Guard specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusProfileType {
    DeepStudy,
    ExamSprint,
    LightReview,
    IntentionalFreeTime,
}

/// Cross-Device Focus Guard Synchronization Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDeviceFocusEvent {
    pub event_id: String,
    pub initiating_device_id: String,
    pub initiating_device_name: String,
    pub is_focus_active: bool,
    pub profile: FocusProfileType,
    pub task_title: String,
    pub start_time_utc: DateTime<Utc>,
    pub duration_minutes: u32,
    pub remaining_seconds: u32,
    pub is_strict_lockdown: bool,
    pub allow_emergency_calls: bool,
    pub paused: bool,
    pub timestamp_utc: DateTime<Utc>,
}

/// Remote emergency bypass request from phone to laptop or vice-versa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusEmergencyBypassRequest {
    pub request_id: String,
    pub requesting_device_id: String,
    pub reason: String,
    pub requested_at: DateTime<Utc>,
    pub auto_approved_contacts: Vec<String>,
}
