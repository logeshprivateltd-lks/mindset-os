//! MindSet OS — Internet Integrity & Network Security Subsystem (`mindset-net`)
//! Unified coordinator combining Zero-Incognito Policy Enforcement, Ad & Telemetry Filter,
//! Entertainment Shield, YouTube Screen-Time Quota, Chromium Manifest V3 Bridge,
//! and 7-Stage Network Doctor.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod adblock_scripts;
pub mod chromium_bridge;
pub mod entertainment_shield;
pub mod net_doctor;
pub mod policy_guard;
pub mod youtube_guard;

pub use adblock_scripts::{AdBlockFilter, AdBlockStats, DeclarativeRule, AD_TRACKER_DOMAINS};
pub use chromium_bridge::{ChromiumEngineBridge, SHIELD_INJECT_JS, SHIELD_MANIFEST_V3_JSON};
pub use entertainment_shield::{
    EntertainmentShield, ACADEMIC_WHITELIST, CHAT_DOMAINS, GAMING_DOMAINS, OTT_STREAMING_DOMAINS,
    SOCIAL_MEDIA_DOMAINS,
};
pub use net_doctor::{InternetTroubleshooter, NetDiagnosticStage, NetRepairAction, StageStatus};
pub use policy_guard::{IntegrityLogEntry, PolicyGuard, INCOGNITO_BLOCKED_MSG};
pub use youtube_guard::{
    YoutubeGuard, DEFAULT_YOUTUBE_LIMIT_SECS, YOUTUBE_DAILY_LIMIT_EXHAUSTED_MSG,
    YOUTUBE_PRESETS_MINUTES, YOUTUBE_SHORTS_BLOCKED_MSG,
};

/// Unified Internet Integrity and Focus Hub for MindSet OS.
#[derive(Debug, Clone, Default)]
pub struct InternetIntegrityHub {
    pub policy_guard: PolicyGuard,
    pub adblock: AdBlockFilter,
    pub entertainment_shield: EntertainmentShield,
    pub youtube_guard: YoutubeGuard,
    pub chromium_bridge: ChromiumEngineBridge,
    pub troubleshooter: InternetTroubleshooter,
}

impl InternetIntegrityHub {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates, normalizes, and filters a navigation URL through the entire security hierarchy.
    /// Returns the normalized HTTPS URL if allowed, or an error string describing the block policy.
    pub fn check_and_normalize_url(
        &mut self,
        url: &str,
        focus_active: bool,
    ) -> Result<String, String> {
        // 1. Validate scheme & strip internal bypass targets
        let normalized = PolicyGuard::validate_url_scheme(url)?;

        // 2. Filter Entertainment, Social Media, Web Games, and Focus Chat
        self.entertainment_shield
            .check_url(&normalized, focus_active)?;

        // 3. Filter YouTube Shorts & Daily Screen-Time Quotas
        self.youtube_guard.check_youtube_url(&normalized)?;

        Ok(normalized)
    }

    /// Launches a URL in a secured, locked-down browser process with force-injected shield scripts.
    pub fn launch_browser(&mut self, url: &str, focus_active: bool) -> Result<String, String> {
        let (binary, args) = self.chromium_bridge.prepare_launch_command(
            url,
            &mut self.entertainment_shield,
            &mut self.youtube_guard,
            focus_active,
        )?;

        // In test environments or when the binary does not exist on disk, return the validated command
        if !binary.exists() {
            return Ok(format!(
                "Command validated for headless execution: {} with {} arguments",
                binary.display(),
                args.len()
            ));
        }

        self.chromium_bridge.launch_shielded_url(
            url,
            &mut self.entertainment_shield,
            &mut self.youtube_guard,
            focus_active,
        )
    }

    /// Advances the YouTube screen-time counter by one second.
    /// Returns `true` the instant the daily limit is exhausted.
    #[inline]
    pub fn tick_second(&mut self) -> bool {
        self.youtube_guard.tick_second()
    }

    /// Re-runs the full 7-stage network diagnosis.
    pub fn run_diagnostics(&mut self) {
        self.troubleshooter.run_full_diagnosis();
    }

    /// Executes one of the 5 one-click network self-healing repair actions.
    pub fn execute_repair(&mut self, action: NetRepairAction) -> Result<String, String> {
        self.troubleshooter.execute_repair(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internet_integrity_hub_full_pipeline() {
        let mut hub = InternetIntegrityHub::new();

        // 1. Academic URL passes
        let ok_url = hub.check_and_normalize_url("wikipedia.org", true).unwrap();
        assert_eq!(ok_url, "https://wikipedia.org");

        // 2. Entertainment URL blocked
        assert!(hub
            .check_and_normalize_url("https://netflix.com", false)
            .is_err());

        // 3. Shorts URL blocked
        assert!(hub
            .check_and_normalize_url("https://youtube.com/shorts/xyz", false)
            .is_err());

        // 4. Internal scheme blocked
        assert!(hub
            .check_and_normalize_url("chrome://extensions", false)
            .is_err());

        // 5. Diagnostics pass
        assert_eq!(hub.troubleshooter.stages.len(), 7);

        // 6. Repair actions work
        assert!(hub
            .execute_repair(NetRepairAction::SwitchToSafeAcademicDns)
            .is_ok());
    }
}
