//! MindSet Session & Privacy Guard Manager
//! Tracks active authentication session, idle auto-lock timeout, Instant Privacy Blur,
//! and temporary 3-minute UAC elevation privileges.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

pub const UAC_ELEVATION_DURATION_MS: i64 = 180_000; // 3 minutes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub session_token: String,
    pub user_id: String,
    pub authenticated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub auth_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    pub is_locked: bool,
    pub is_privacy_blurred: bool,
    pub is_uac_elevated: bool,
    pub active_user: Option<String>,
    pub auth_method: Option<String>,
    pub auto_lock_minutes: u32,
}

pub struct SessionManager {
    is_locked: bool,
    is_privacy_blurred: bool,
    active_session: Option<AuthSession>,
    last_activity_ms: i64,
    auto_lock_minutes: u32,
    uac_elevated_until_ms: Option<i64>,
}

impl SessionManager {
    pub fn new() -> Self {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        Self {
            is_locked: true,
            is_privacy_blurred: false,
            active_session: None,
            last_activity_ms: now_ms,
            auto_lock_minutes: 5,
            uac_elevated_until_ms: None,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn is_privacy_blurred(&self) -> bool {
        self.is_privacy_blurred
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
        self.active_session = None;
        self.uac_elevated_until_ms = None;
        info!("System locked (Smart Lock engaged)");
    }

    pub fn unlock(&mut self, session: AuthSession) {
        self.is_locked = false;
        self.is_privacy_blurred = false;
        self.active_session = Some(session);
        self.touch_activity();
        info!("System unlocked successfully");
    }

    pub fn toggle_privacy_blur(&mut self) -> bool {
        self.is_privacy_blurred = !self.is_privacy_blurred;
        info!("Instant Privacy Blur set to: {}", self.is_privacy_blurred);
        self.is_privacy_blurred
    }

    pub fn set_privacy_blur(&mut self, blurred: bool) {
        self.is_privacy_blurred = blurred;
    }

    pub fn touch_activity(&mut self) {
        self.last_activity_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
    }

    /// Checks whether user inactivity has exceeded auto_lock_minutes
    pub fn check_idle_timeout(&mut self) -> bool {
        if self.is_locked {
            return false;
        }

        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        let idle_limit_ms = (self.auto_lock_minutes as i64) * 60 * 1000;
        if now_ms - self.last_activity_ms > idle_limit_ms {
            self.lock();
            info!(
                "Idle auto-lock triggered after {} minutes of inactivity",
                self.auto_lock_minutes
            );
            true
        } else {
            false
        }
    }

    /// Grants temporary 3-minute UAC elevation token
    pub fn elevate_uac(&mut self) -> String {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        self.uac_elevated_until_ms = Some(now_ms + UAC_ELEVATION_DURATION_MS);
        let uac_token = format!("uac-{}", uuid::Uuid::new_v4());
        info!("UAC Admin Elevation granted for 3 minutes");
        uac_token
    }

    /// Checks if temporary UAC elevation is currently valid
    pub fn is_uac_elevated(&self) -> bool {
        if let Some(elevated_until) = self.uac_elevated_until_ms {
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis() as i64)
                .unwrap_or(0);

            now_ms < elevated_until
        } else {
            false
        }
    }

    pub fn get_status(&self) -> AuthStatus {
        AuthStatus {
            is_locked: self.is_locked,
            is_privacy_blurred: self.is_privacy_blurred,
            is_uac_elevated: self.is_uac_elevated(),
            active_user: self.active_session.as_ref().map(|s| s.user_id.clone()),
            auth_method: self.active_session.as_ref().map(|s| s.auth_method.clone()),
            auto_lock_minutes: self.auto_lock_minutes,
        }
    }

    pub fn create_session(user_id: &str, method: &str) -> AuthSession {
        AuthSession {
            session_token: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            authenticated_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(8),
            auth_method: method.to_string(),
        }
    }
}
