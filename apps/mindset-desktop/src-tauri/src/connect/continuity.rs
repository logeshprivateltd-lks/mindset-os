//! MindSet Connect — Desktop Module 4: Continuity Engine (`continuity.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use connect_protocol::{
    ContinuityItemType, ContinuityStateHandoff, CrossDeviceFocusEvent, FocusEmergencyBypassRequest,
    FocusProfileType,
};
use std::sync::Mutex;
use uuid::Uuid;

pub struct DesktopContinuityManager {
    current_handoff: Mutex<Option<ContinuityStateHandoff>>,
    active_focus_event: Mutex<Option<CrossDeviceFocusEvent>>,
}

impl DesktopContinuityManager {
    pub fn new() -> Self {
        let initial_handoff = ContinuityStateHandoff {
            session_id: "ses-handoff-01".to_string(),
            source_device_id: "mindset-host-laptop-01".to_string(),
            source_device_name: "MindSet Laptop (Core)".to_string(),
            item_type: ContinuityItemType::PdfTextbook,
            title: "Physics Ch 4: Ray Optics and Optical Instruments".to_string(),
            resource_identifier: "vault/my-laptop/Physics_Ray_Optics_HSC.pdf".to_string(),
            last_read_page: Some(84),
            total_pages: Some(210),
            scroll_offset_ratio: Some(0.42),
            draft_content: None,
            active_subject: Some("Physics".to_string()),
            timestamp_utc: Utc::now() - chrono::Duration::minutes(5),
        };

        let initial_focus = CrossDeviceFocusEvent {
            event_id: "fcs-event-01".to_string(),
            initiating_device_id: "mindset-host-laptop-01".to_string(),
            initiating_device_name: "MindSet Laptop".to_string(),
            is_focus_active: true,
            profile: FocusProfileType::DeepStudy,
            task_title: "Master Wave Optics Young's Double Slit Derivation".to_string(),
            start_time_utc: Utc::now() - chrono::Duration::minutes(20),
            duration_minutes: 45,
            remaining_seconds: 1500, // 25 min remaining
            is_strict_lockdown: true,
            allow_emergency_calls: true,
            paused: false,
            timestamp_utc: Utc::now(),
        };

        Self {
            current_handoff: Mutex::new(Some(initial_handoff)),
            active_focus_event: Mutex::new(Some(initial_focus)),
        }
    }

    /// Retrieve the current "Continue Where You Left Off" handoff state
    pub fn get_continuity_state(&self) -> Option<ContinuityStateHandoff> {
        self.current_handoff.lock().unwrap().clone()
    }

    /// Broadcast active state change (e.g., student opened textbook or jumped to page 92)
    pub fn broadcast_continuity_state(
        &self,
        item_type: ContinuityItemType,
        title: &str,
        resource_identifier: &str,
        last_read_page: Option<u32>,
        total_pages: Option<u32>,
        subject: Option<String>,
    ) -> ContinuityStateHandoff {
        let handoff = ContinuityStateHandoff {
            session_id: format!("ses-{}", Uuid::new_v4().simple()),
            source_device_id: "mindset-host-laptop-01".to_string(),
            source_device_name: "MindSet Laptop".to_string(),
            item_type,
            title: title.to_string(),
            resource_identifier: resource_identifier.to_string(),
            last_read_page,
            total_pages,
            scroll_offset_ratio: Some(0.0),
            draft_content: None,
            active_subject: subject,
            timestamp_utc: Utc::now(),
        };

        *self.current_handoff.lock().unwrap() = Some(handoff.clone());
        handoff
    }

    /// Retrieve active cross-device Focus Guard state
    pub fn get_active_focus(&self) -> Option<CrossDeviceFocusEvent> {
        self.active_focus_event.lock().unwrap().clone()
    }

    /// Synchronize Focus Guard state across Laptop and Phone
    pub fn sync_focus_session(
        &self,
        is_active: bool,
        profile: FocusProfileType,
        task_title: &str,
        duration_minutes: u32,
        remaining_seconds: u32,
    ) -> CrossDeviceFocusEvent {
        let event = CrossDeviceFocusEvent {
            event_id: format!("fcs-{}", Uuid::new_v4().simple()),
            initiating_device_id: "mindset-host-laptop-01".to_string(),
            initiating_device_name: "MindSet Laptop".to_string(),
            is_focus_active: is_active,
            profile,
            task_title: task_title.to_string(),
            start_time_utc: Utc::now(),
            duration_minutes,
            remaining_seconds,
            is_strict_lockdown: true,
            allow_emergency_calls: true,
            paused: false,
            timestamp_utc: Utc::now(),
        };

        *self.active_focus_event.lock().unwrap() = Some(event.clone());
        event
    }

    /// Trigger emergency unlock request
    #[allow(dead_code)]
    pub fn trigger_emergency_bypass(&self, reason: &str) -> FocusEmergencyBypassRequest {
        FocusEmergencyBypassRequest {
            request_id: format!("req-{}", Uuid::new_v4().simple()),
            requesting_device_id: "dev-phone-galaxy-s24".to_string(),
            reason: reason.to_string(),
            requested_at: Utc::now(),
            auto_approved_contacts: vec!["Mom".to_string(), "Dad".to_string(), "School Office".to_string()],
        }
    }
}
