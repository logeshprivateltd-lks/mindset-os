//! Centralized 10-Pillar Personalization Store, SQLite WAL Persistence, and 11-Step Onboarding.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::db::DbError;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

/// Individual memory unit remembered by Maindy AI with user permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMemoryItem {
    pub id: String,
    pub category: String, // "Learning Style", "Goal", "Topic Preference", "Schedule"
    pub content: String,
    pub created_at: String,
}

/// Adaptive AI suggestion presented transparently to the student for approval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveSuggestion {
    pub id: String,
    pub title: String,
    pub recommendation: String,
    pub rationale: String,
    pub target_setting: String,
    pub proposed_value: String,
    pub is_approved: bool,
    pub is_dismissed: bool,
}

fn default_workspaces_enabled() -> [bool; 4] {
    [true, true, true, true]
}

fn default_widgets_enabled() -> [bool; 4] {
    [true, true, true, false]
}

/// Tab 1: Profile & Identity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub preferred_name: String,
    pub avatar_id: usize,
    pub language: String,      // "English", "Tamil", "Tanglish", "Hindi"
    pub user_category: String, // "Student", "Professional", "Entrepreneur", "Creator", "Freelancer"
    pub personal_goals: Vec<String>,
    pub time_format_24h: bool,
    #[serde(default)]
    pub academic_stage: usize,
    #[serde(default)]
    pub study_target_hours_idx: usize,
    #[serde(default)]
    pub target_identity_label: String,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            preferred_name: "Student Scholar".to_string(),
            avatar_id: 0,
            language: "English".to_string(),
            user_category: "Student".to_string(),
            personal_goals: vec![
                "Score 95%+ in Class 12 Board Examinations".to_string(),
                "Master Electromagnetism & Calculus Integration".to_string(),
                "Sustain 4 deep focus blocks daily without burnout".to_string(),
            ],
            time_format_24h: false,
            academic_stage: 1,
            study_target_hours_idx: 1,
            target_identity_label: "Dr. Arjun, MBBS 2029".to_string(),
        }
    }
}

/// Tab 2: Appearance & Liquid Glass
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme_mode: String, // "Dark", "Light", "Auto"
    pub theme_preset: u32, // 0: Atmospheric Blue Glass, 1: Midnight, 2: Aurora, 3: Minimal, 4: Emerald Zen, 5: Custom
    pub accent_color: [f32; 4],
    pub wallpaper_preset: u32, // 0..4
    pub transparency: f32,     // 0.4..=0.95
    pub blur_radius: f32,      // 0..=32px
    pub border_opacity: f32,   // 0.2..=1.0
    pub shadow_intensity: f32, // 0.0..=1.0
    pub ui_scale: f32,         // 0.80..=1.50 (default 1.15)
    pub auto_wallpaper_accent: bool,
    #[serde(default)]
    pub blur_quality: usize, // 0: Dual-Kawase 4-Pass, 1: Standard 2-Pass
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme_mode: "Dark".to_string(),
            theme_preset: 0,
            accent_color: [0.145, 0.388, 0.922, 1.0], // Blue #2563EB
            wallpaper_preset: 1,                      // Aurora Night
            transparency: 0.88,
            blur_radius: 16.0,
            border_opacity: 0.55,
            shadow_intensity: 0.40,
            ui_scale: 1.15,
            auto_wallpaper_accent: false,
            blur_quality: 0,
        }
    }
}

/// Tab 3: Desktop, Widgets & Dock
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopConfig {
    pub enabled_widgets: Vec<String>,
    pub dock_position: String, // "Bottom", "Left", "Right"
    pub dock_autohide: bool,
    pub dock_icon_size: f32,
    pub show_date: bool,
    pub show_time: bool,
    pub show_wifi: bool,
    pub show_bluetooth: bool,
    pub show_battery: bool,
    pub show_window_count: bool,
    pub show_notifications: bool,
    #[serde(default = "default_workspaces_enabled")]
    pub workspaces_enabled: [bool; 4],
    #[serde(default = "default_widgets_enabled")]
    pub widgets_enabled: [bool; 4],
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            enabled_widgets: vec![
                "Digital Clock".to_string(),
                "Today's Study Schedule".to_string(),
                "Performance Sparkline".to_string(),
                "Focus Session Timer".to_string(),
                "Academic Goals".to_string(),
                "System Performance".to_string(),
            ],
            dock_position: "Bottom".to_string(),
            dock_autohide: false,
            dock_icon_size: 42.0,
            show_date: true,
            show_time: true,
            show_wifi: true,
            show_bluetooth: true,
            show_battery: true,
            show_window_count: true,
            show_notifications: true,
            workspaces_enabled: [true, true, true, true],
            widgets_enabled: [true, true, true, false],
        }
    }
}

/// Tab 4: AI Personalization, Memory & 3 Modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiPersonalizationConfig {
    pub active_mode: String,     // "Tutor", "Coach", "Planner"
    pub personality: String, // "Educational", "Supportive", "Professional", "Friendly", "Direct", "Motivational", "Calm"
    pub response_detail: String, // "Examples", "Detailed", "Short", "Bullets"
    pub enable_memory: bool,
    pub memories: Vec<AiMemoryItem>,
    pub require_citations: bool,
}

impl Default for AiPersonalizationConfig {
    fn default() -> Self {
        Self {
            active_mode: "Tutor".to_string(),
            personality: "Educational".to_string(),
            response_detail: "Detailed".to_string(),
            enable_memory: true,
            memories: vec![
                AiMemoryItem {
                    id: "mem_01".to_string(),
                    category: "Learning Style".to_string(),
                    content: "Prefers derivation breakdowns from first principles before reviewing formula shortcuts.".to_string(),
                    created_at: Utc::now().to_rfc3339(),
                },
                AiMemoryItem {
                    id: "mem_02".to_string(),
                    category: "Target Exam".to_string(),
                    content: "Preparing for CBSE Class 12 Board Examination & JEE Entrance in Physics and Calculus.".to_string(),
                    created_at: Utc::now().to_rfc3339(),
                },
            ],
            require_citations: true,
        }
    }
}

/// Tab 5: Daily Routine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutineConfig {
    pub wake_time: String,
    pub sleep_time: String,
    pub study_hours_per_day: f32,
    pub break_duration_min: u32,
    pub peak_productivity_period: String, // "Morning (6 AM - 10 AM)", "Afternoon", "Night (8 PM - 12 AM)"
}

impl Default for RoutineConfig {
    fn default() -> Self {
        Self {
            wake_time: "06:30 AM".to_string(),
            sleep_time: "11:00 PM".to_string(),
            study_hours_per_day: 6.0,
            break_duration_min: 10,
            peak_productivity_period: "Morning (6 AM - 10 AM)".to_string(),
        }
    }
}

/// Tab 6: Notifications & Distraction Guard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsConfig {
    pub system_alerts: bool,
    pub planner_reminders: bool,
    pub focus_alerts: bool,
    pub habit_reminders: bool,
    pub ai_insights: bool,
    pub quiet_hours_enabled: bool,
    pub dnd_enabled: bool,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            system_alerts: true,
            planner_reminders: true,
            focus_alerts: true,
            habit_reminders: true,
            ai_insights: true,
            quiet_hours_enabled: false,
            dnd_enabled: false,
        }
    }
}

/// Operating Mode for MindSet OS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OsMode {
    #[default]
    Standard,
    Mindful,
    ExamLock,
    SleepWindDown,
}

impl OsMode {
    pub fn all() -> &'static [OsMode] {
        &[
            OsMode::Standard,
            OsMode::Mindful,
            OsMode::ExamLock,
            OsMode::SleepWindDown,
        ]
    }

    pub fn pill_label(&self) -> &'static str {
        match self {
            Self::Standard => "[Mode: Standard ▾]",
            Self::Mindful => "[Mode: Mindful ✦]",
            Self::ExamLock => "[Mode: Exam Lock 🔒]",
            Self::SleepWindDown => "[Mode: Sleep 🌙]",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Standard => "All apps and desktop tools available.",
            Self::Mindful => "Focus shield active: restricts distracting apps on Home and Dock.",
            Self::ExamLock => "High-stakes study mode: full lockdown of non-academic tools.",
            Self::SleepWindDown => "Circadian wind-down: night warmth 100%, alerts silenced.",
        }
    }
}

/// Tab 7: Productivity & Workspaces
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductivityConfig {
    pub focus_duration_min: u32, // 15, 25, 45, 60
    pub break_duration_min: u32,
    pub long_break_min: u32,
    pub cycles_before_long_break: u32,
    pub task_view_mode: String, // "List View", "Board View", "Calendar View"
    pub workspace_presets: Vec<String>,
    pub current_mode: OsMode,
    pub allowed_apps_in_mindful: Vec<String>,
}

impl Default for ProductivityConfig {
    fn default() -> Self {
        Self {
            focus_duration_min: 25,
            break_duration_min: 5,
            long_break_min: 20,
            cycles_before_long_break: 4,
            task_view_mode: "List View".to_string(),
            workspace_presets: vec![
                "Study Session".to_string(),
                "Active Problem Solving".to_string(),
                "Creative / Research".to_string(),
                "Personal / Revision".to_string(),
            ],
            current_mode: OsMode::Standard,
            allowed_apps_in_mindful: vec![
                "StudyReader".to_string(),
                "Tasks".to_string(),
                "LayaAiChat".to_string(),
                "SystemMonitor".to_string(),
            ],
        }
    }
}

impl ProductivityConfig {
    pub fn is_app_allowed(&self, app_id: &str) -> bool {
        if self.current_mode == OsMode::Standard {
            true
        } else {
            self.allowed_apps_in_mindful.iter().any(|a| a == app_id)
        }
    }

    pub fn toggle_app_allowed(&mut self, app_id: &str) {
        if let Some(idx) = self
            .allowed_apps_in_mindful
            .iter()
            .position(|a| a == app_id)
        {
            self.allowed_apps_in_mindful.remove(idx);
        } else {
            self.allowed_apps_in_mindful.push(app_id.to_string());
        }
    }
}

/// Tab 8: Accessibility & Sensory Settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub high_contrast_mode: bool,
    pub color_blind_safe: bool,
    pub larger_controls: bool,
    pub reduce_motion: bool,
    pub disable_animations: bool,
    pub reduce_transparency: bool,
    pub visual_audio_captions: bool,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            high_contrast_mode: false,
            color_blind_safe: false,
            larger_controls: false,
            reduce_motion: false,
            disable_animations: false,
            reduce_transparency: false,
            visual_audio_captions: true,
        }
    }
}

/// Tab 9: Sound, Voice & Wireless Audio
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundConfig {
    pub master_volume: f32,         // 0.0..=1.0
    pub ambient_soundscape: String, // "Off", "Soft Rain", "Focus Tone", "Nature", "White Noise"
    pub tactile_click_sound: bool,
    pub ai_voice_speed: f32, // 0.8..=1.4
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.85,
            ambient_soundscape: "Focus Tone".to_string(),
            tactile_click_sound: true,
            ai_voice_speed: 1.0,
        }
    }
}

/// Tab 10: Privacy, Permissions & Adaptive Suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub local_ai_mode: bool,
    pub perm_file_vault: bool,
    pub perm_microphone: bool,
    pub perm_camera: bool,
    pub perm_network_sync: bool,
    pub perm_notifications: bool,
    pub adaptive_suggestions: Vec<AdaptiveSuggestion>,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            local_ai_mode: true,
            perm_file_vault: true,
            perm_microphone: false,
            perm_camera: false,
            perm_network_sync: true,
            perm_notifications: true,
            adaptive_suggestions: vec![
                AdaptiveSuggestion {
                    id: "sug_01".to_string(),
                    title: "Enable Night Warmth at 7:00 PM".to_string(),
                    recommendation: "Shift display to warm amber tones in the evening to reduce blue-light eye strain.".to_string(),
                    rationale: "Detected evening study sessions past 7:30 PM.".to_string(),
                    target_setting: "night_warmth".to_string(),
                    proposed_value: "0.65".to_string(),
                    is_approved: false,
                    is_dismissed: false,
                },
                AdaptiveSuggestion {
                    id: "sug_02".to_string(),
                    title: "Extend Focus Block to 45m".to_string(),
                    recommendation: "Increase Pomodoro block from 25m to 45m for advanced Calculus derivations.".to_string(),
                    rationale: "Average study session duration exceeded 40 minutes over the last 3 days.".to_string(),
                    target_setting: "focus_duration".to_string(),
                    proposed_value: "45".to_string(),
                    is_approved: false,
                    is_dismissed: false,
                },
            ],
        }
    }
}

/// Centralized 10-Pillar Personalization Configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PersonalizationStore {
    pub profile: ProfileConfig,
    pub appearance: AppearanceConfig,
    pub desktop: DesktopConfig,
    pub ai: AiPersonalizationConfig,
    pub routine: RoutineConfig,
    pub notifications: NotificationsConfig,
    pub productivity: ProductivityConfig,
    pub accessibility: AccessibilityConfig,
    pub sound: SoundConfig,
    pub privacy: PrivacyConfig,
    pub updated_at: String,
}

impl PersonalizationStore {
    pub fn new() -> Self {
        Self {
            profile: ProfileConfig::default(),
            appearance: AppearanceConfig::default(),
            desktop: DesktopConfig::default(),
            ai: AiPersonalizationConfig::default(),
            routine: RoutineConfig::default(),
            notifications: NotificationsConfig::default(),
            productivity: ProductivityConfig::default(),
            accessibility: AccessibilityConfig::default(),
            sound: SoundConfig::default(),
            privacy: PrivacyConfig::default(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn reset_to_defaults(&mut self) {
        *self = Self::new();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Persists the entire personalization store into local_core.sqlite.
    pub fn save_to_sqlite(&self, conn: &Connection) -> Result<(), DbError> {
        let json_data = self
            .to_json()
            .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS system_personalization_store (
                key TEXT PRIMARY KEY,
                json_payload TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
            [],
        )?;

        conn.execute(
            r#"
            INSERT INTO system_personalization_store (key, json_payload, updated_at)
            VALUES ('active_config', ?1, ?2)
            ON CONFLICT(key) DO UPDATE SET
                json_payload = excluded.json_payload,
                updated_at = excluded.updated_at
            "#,
            params![json_data, now],
        )?;

        Ok(())
    }

    /// Loads the active personalization store from local_core.sqlite (or fallback default).
    pub fn load_from_sqlite(conn: &Connection) -> Result<Self, DbError> {
        let res: Result<String, rusqlite::Error> = conn.query_row(
            "SELECT json_payload FROM system_personalization_store WHERE key = 'active_config'",
            [],
            |row| row.get(0),
        );

        match res {
            Ok(json_payload) => {
                Self::from_json(&json_payload).map_err(|e| DbError::MigrationFailed(e.to_string()))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let default_store = Self::new();
                let _ = default_store.save_to_sqlite(conn);
                Ok(default_store)
            }
            Err(_e) => {
                // Table might not exist yet
                let default_store = Self::new();
                let _ = default_store.save_to_sqlite(conn);
                Ok(default_store)
            }
        }
    }

    pub fn approve_adaptive_suggestion(&mut self, id: &str) -> bool {
        if let Some(sug) = self
            .privacy
            .adaptive_suggestions
            .iter_mut()
            .find(|s| s.id == id)
        {
            sug.is_approved = true;
            match sug.target_setting.as_str() {
                "night_warmth" => {
                    if let Ok(val) = sug.proposed_value.parse::<f32>() {
                        // Applied to appearance
                        self.appearance.border_opacity = val;
                    }
                }
                "focus_duration" => {
                    if let Ok(val) = sug.proposed_value.parse::<u32>() {
                        self.productivity.focus_duration_min = val;
                    }
                }
                _ => {}
            }
            true
        } else {
            false
        }
    }

    pub fn dismiss_adaptive_suggestion(&mut self, id: &str) -> bool {
        if let Some(sug) = self
            .privacy
            .adaptive_suggestions
            .iter_mut()
            .find(|s| s.id == id)
        {
            sug.is_dismissed = true;
            true
        } else {
            false
        }
    }
}

/// 11-Step Initial User Onboarding Wizard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingWizardState {
    pub is_open: bool,
    pub current_step: usize,
    pub total_steps: usize,
    pub step_titles: Vec<String>,
}

impl Default for OnboardingWizardState {
    fn default() -> Self {
        Self {
            is_open: false,
            current_step: 0,
            total_steps: 11,
            step_titles: vec![
                "Welcome & Language".into(),
                "Theme & Appearance".into(),
                "Your Preferred Name".into(),
                "Role & Study Category".into(),
                "Academic Goals".into(),
                "Daily Routine & Sleep".into(),
                "Maindy AI Tutor Mode".into(),
                "AI Personality & Tone".into(),
                "Notification Guard".into(),
                "Sovereign Local Privacy".into(),
                "Setup Complete!".into(),
            ],
        }
    }
}

impl OnboardingWizardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.current_step = 0;
    }

    pub fn next_step(&mut self) -> bool {
        if self.current_step + 1 < self.total_steps {
            self.current_step += 1;
            true
        } else {
            self.is_open = false;
            false
        }
    }

    pub fn prev_step(&mut self) -> bool {
        if self.current_step > 0 {
            self.current_step -= 1;
            true
        } else {
            false
        }
    }

    pub fn current_title(&self) -> &str {
        self.step_titles
            .get(self.current_step)
            .map(|s| s.as_str())
            .unwrap_or("Onboarding")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personalization_store_serialization_and_defaults() {
        let mut store = PersonalizationStore::new();
        assert_eq!(store.profile.language, "English");
        assert_eq!(store.appearance.ui_scale, 1.15);
        assert_eq!(store.productivity.focus_duration_min, 25);
        assert_eq!(store.ai.active_mode, "Tutor");

        // Mutate
        store.profile.preferred_name = "Arjun Raman".to_string();
        store.profile.language = "Tamil".to_string();

        let json = store.to_json().expect("Failed to serialize");
        assert!(json.contains("Arjun Raman"));
        assert!(json.contains("Tamil"));

        let restored = PersonalizationStore::from_json(&json).expect("Failed to deserialize");
        assert_eq!(restored.profile.preferred_name, "Arjun Raman");
        assert_eq!(restored.profile.language, "Tamil");

        // Reset to defaults
        store.reset_to_defaults();
        assert_eq!(store.profile.preferred_name, "Student Scholar");
    }

    #[test]
    fn test_adaptive_suggestion_approval() {
        let mut store = PersonalizationStore::new();
        assert_eq!(store.productivity.focus_duration_min, 25);

        assert!(store.approve_adaptive_suggestion("sug_02"));
        assert_eq!(store.productivity.focus_duration_min, 45);

        assert!(store.dismiss_adaptive_suggestion("sug_01"));
        assert!(store.privacy.adaptive_suggestions[0].is_dismissed);
    }

    #[test]
    fn test_onboarding_wizard_step_traversal() {
        let mut wizard = OnboardingWizardState::new();
        assert!(!wizard.is_open);
        wizard.open();
        assert!(wizard.is_open);
        assert_eq!(wizard.current_step, 0);
        assert_eq!(wizard.current_title(), "Welcome & Language");

        for step in 1..11 {
            assert!(wizard.next_step());
            assert_eq!(wizard.current_step, step);
        }

        // Final step completes
        assert!(!wizard.next_step());
        assert!(!wizard.is_open);
    }

    #[test]
    fn test_os_mode_and_mindful_app_filtering() {
        let mut prod = ProductivityConfig::default();
        assert_eq!(prod.current_mode, OsMode::Standard);
        // In standard mode, all apps allowed
        assert!(prod.is_app_allowed("SearchBrowser"));
        assert!(prod.is_app_allowed("StudyReader"));

        // Switch to Mindful Mode
        prod.current_mode = OsMode::Mindful;
        assert_eq!(prod.current_mode.pill_label(), "[Mode: Mindful ✦]");
        // Study tools allowed by default
        assert!(prod.is_app_allowed("StudyReader"));
        assert!(prod.is_app_allowed("Tasks"));
        // Distracting apps restricted by default
        assert!(!prod.is_app_allowed("SearchBrowser"));

        // Toggle permission
        prod.toggle_app_allowed("SearchBrowser");
        assert!(prod.is_app_allowed("SearchBrowser"));
        prod.toggle_app_allowed("SearchBrowser");
        assert!(!prod.is_app_allowed("SearchBrowser"));
    }
}
