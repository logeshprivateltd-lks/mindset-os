//! iOS-Style Focus & Mindful Mode Switcher with Custom Home Screen & App Restriction Editor.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Switchable OS Focus & Mindful Operating Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OsFocusMode {
    /// Standard: All 8 apps, all widgets, and all workspaces enabled
    #[default]
    Standard,
    /// Mindful: Distraction-free yellow-amber glow; only user-approved Home Screen apps & widgets are visible
    Mindful,
    /// StudyDeep: Only StudyReader, MaindyAiChat, FileVault, and TasksGoals allowed on Home Screen
    StudyDeep,
    /// WindDown: Warm amber filter, notifications silenced, minimal clock + reflection view
    WindDown,
}

impl OsFocusMode {
    pub fn all() -> [OsFocusMode; 4] {
        [
            Self::Standard,
            Self::Mindful,
            Self::StudyDeep,
            Self::WindDown,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Standard => "Standard Mode",
            Self::Mindful => "Mindful Focus",
            Self::StudyDeep => "Deep Study Block",
            Self::WindDown => "Evening Wind-Down",
        }
    }

    pub fn pill_label(&self) -> &'static str {
        match self {
            Self::Standard => "Mode: Standard ▾",
            Self::Mindful => "Mode: Mindful ✦",
            Self::StudyDeep => "Mode: StudyDeep 🔬",
            Self::WindDown => "Mode: WindDown 🌙",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Standard => "All 8 applications, workspace tools, and unrestricted multitasking.",
            Self::Mindful => "Filtered Home Desktop & Dock with golden amber focus protection.",
            Self::StudyDeep => "Strict distraction lockdown for intensive exam preparation.",
            Self::WindDown => {
                "70% blue-light attenuation and serene reflection for restorative rest."
            }
        }
    }
}

/// Custom Home Screen & Page Configuration for Mindful Mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindfulHomeConfig {
    /// Applications allowed to be visible on the Home Screen Desktop Grid & Dock
    pub allowed_apps: HashSet<String>,
    /// Show the left glanceable study widgets
    pub show_left_widgets: bool,
    /// Show the desktop shortcut grid
    pub show_desktop_icons: bool,
    /// Show top global search bar
    pub show_search_bar: bool,
    /// Show bottom application dock
    pub show_bottom_dock: bool,
}

impl Default for MindfulHomeConfig {
    fn default() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("StudyReader".to_string());
        allowed.insert("LayaAiChat".to_string());
        allowed.insert("TasksGoals".to_string());

        Self {
            allowed_apps: allowed,
            show_left_widgets: true,
            show_desktop_icons: true,
            show_search_bar: true,
            show_bottom_dock: true,
        }
    }
}

/// Mindful Mode Runtime Engine coordinating app restrictions, temporary 5m unlocks, and widget visibility.
#[derive(Debug, Clone)]
pub struct MindfulModeEngine {
    pub mode: OsFocusMode,
    pub config: MindfulHomeConfig,
    /// Temporary app bypasses (e.g. "Allow for 5m") keyed by app_id -> expiry Instant
    pub temporary_unlocks: HashMap<String, Instant>,
}

impl Default for MindfulModeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MindfulModeEngine {
    pub fn new() -> Self {
        Self {
            mode: OsFocusMode::Standard,
            config: MindfulHomeConfig::default(),
            temporary_unlocks: HashMap::new(),
        }
    }

    #[inline]
    pub fn mode(&self) -> OsFocusMode {
        self.mode
    }

    #[inline]
    pub fn set_mode(&mut self, mode: OsFocusMode) {
        self.mode = mode;
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.mode != OsFocusMode::Standard
    }

    pub fn cycle_mode(&mut self) -> OsFocusMode {
        self.mode = match self.mode {
            OsFocusMode::Standard => OsFocusMode::Mindful,
            OsFocusMode::Mindful => OsFocusMode::StudyDeep,
            OsFocusMode::StudyDeep => OsFocusMode::WindDown,
            OsFocusMode::WindDown => OsFocusMode::Standard,
        };
        self.mode
    }

    /// Check if a specific app is permitted to appear on Home or launch under the current mode
    pub fn is_app_allowed(&self, app_id: &str) -> bool {
        if self.mode == OsFocusMode::Standard {
            return true;
        }

        // Check if temporarily unlocked
        if self.is_temporarily_unlocked(app_id) {
            return true;
        }

        match self.mode {
            OsFocusMode::Standard => true,
            OsFocusMode::Mindful => self.config.allowed_apps.contains(app_id),
            OsFocusMode::StudyDeep => {
                matches!(
                    app_id,
                    "StudyReader" | "LayaAiChat" | "FileVault" | "TasksGoals"
                )
            }
            OsFocusMode::WindDown => {
                matches!(app_id, "TasksGoals" | "LayaAiChat")
            }
        }
    }

    /// Grant a temporary bypass (e.g., 5 minutes) to a restricted app
    pub fn allow_app_for_duration(&mut self, app_id: &str, duration: Duration) {
        let expiry = Instant::now() + duration;
        self.temporary_unlocks.insert(app_id.to_string(), expiry);
    }

    /// Check whether a temporary unlock has not expired
    pub fn is_temporarily_unlocked(&self, app_id: &str) -> bool {
        if let Some(expiry) = self.temporary_unlocks.get(app_id) {
            Instant::now() < *expiry
        } else {
            false
        }
    }

    /// Clean up expired unlock entries
    pub fn clean_expired_unlocks(&mut self) {
        let now = Instant::now();
        self.temporary_unlocks.retain(|_, expiry| *expiry > now);
    }

    /// Toggle app permission in Mindful Mode configuration
    pub fn toggle_app_allowed(&mut self, app_id: &str) {
        if self.config.allowed_apps.contains(app_id) {
            self.config.allowed_apps.remove(app_id);
        } else {
            self.config.allowed_apps.insert(app_id.to_string());
        }
    }

    pub fn set_app_allowed(&mut self, app_id: &str, allowed: bool) {
        if allowed {
            self.config.allowed_apps.insert(app_id.to_string());
        } else {
            self.config.allowed_apps.remove(app_id);
        }
    }

    #[inline]
    pub fn toggle_left_widgets(&mut self) {
        self.config.show_left_widgets = !self.config.show_left_widgets;
    }

    #[inline]
    pub fn toggle_desktop_icons(&mut self) {
        self.config.show_desktop_icons = !self.config.show_desktop_icons;
    }

    #[inline]
    pub fn toggle_search_bar(&mut self) {
        self.config.show_search_bar = !self.config.show_search_bar;
    }

    #[inline]
    pub fn toggle_bottom_dock(&mut self) {
        self.config.show_bottom_dock = !self.config.show_bottom_dock;
    }

    /// Returns whether left study widgets should be rendered
    pub fn should_show_left_widgets(&self) -> bool {
        match self.mode {
            OsFocusMode::Standard => true,
            OsFocusMode::WindDown => false,
            _ => self.config.show_left_widgets,
        }
    }

    /// Returns whether desktop app shortcut icons should be rendered
    pub fn should_show_desktop_icons(&self) -> bool {
        match self.mode {
            OsFocusMode::Standard => true,
            OsFocusMode::WindDown => false,
            _ => self.config.show_desktop_icons,
        }
    }

    /// Returns whether the top search bar should be rendered
    pub fn should_show_search_bar(&self) -> bool {
        match self.mode {
            OsFocusMode::Standard => true,
            OsFocusMode::WindDown => false,
            _ => self.config.show_search_bar,
        }
    }

    /// Returns whether the bottom application dock should be rendered
    pub fn should_show_bottom_dock(&self) -> bool {
        match self.mode {
            OsFocusMode::Standard => true,
            _ => self.config.show_bottom_dock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_focus_mode_transitions() {
        let mut engine = MindfulModeEngine::new();
        assert_eq!(engine.mode(), OsFocusMode::Standard);
        assert_eq!(engine.cycle_mode(), OsFocusMode::Mindful);
        assert_eq!(engine.cycle_mode(), OsFocusMode::StudyDeep);
        assert_eq!(engine.cycle_mode(), OsFocusMode::WindDown);
        assert_eq!(engine.cycle_mode(), OsFocusMode::Standard);
    }

    #[test]
    fn test_mindful_mode_app_restriction_and_toggle() {
        let mut engine = MindfulModeEngine::new();
        engine.set_mode(OsFocusMode::Mindful);

        // By default: StudyReader, LayaAiChat, TasksGoals allowed
        assert!(engine.is_app_allowed("StudyReader"));
        assert!(engine.is_app_allowed("LayaAiChat"));
        assert!(engine.is_app_allowed("TasksGoals"));

        // Other apps restricted
        assert!(!engine.is_app_allowed("Settings"));
        assert!(!engine.is_app_allowed("SystemMonitor"));
        assert!(!engine.is_app_allowed("SearchBrowser"));

        // Toggle Settings to allowed
        engine.toggle_app_allowed("Settings");
        assert!(engine.is_app_allowed("Settings"));

        // Toggle back off
        engine.toggle_app_allowed("Settings");
        assert!(!engine.is_app_allowed("Settings"));
    }

    #[test]
    fn test_temporary_unlock_5_minutes() {
        let mut engine = MindfulModeEngine::new();
        engine.set_mode(OsFocusMode::Mindful);
        assert!(!engine.is_app_allowed("Settings"));

        // Unlock Settings for 5 minutes
        engine.allow_app_for_duration("Settings", Duration::from_secs(300));
        assert!(engine.is_temporarily_unlocked("Settings"));
        assert!(engine.is_app_allowed("Settings"));

        // An expired duration should relock
        engine.allow_app_for_duration("Settings", Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(5));
        assert!(!engine.is_temporarily_unlocked("Settings"));
        assert!(!engine.is_app_allowed("Settings"));
    }

    #[test]
    fn test_study_deep_and_wind_down_presets() {
        let mut engine = MindfulModeEngine::new();

        // StudyDeep
        engine.set_mode(OsFocusMode::StudyDeep);
        assert!(engine.is_app_allowed("StudyReader"));
        assert!(engine.is_app_allowed("LayaAiChat"));
        assert!(engine.is_app_allowed("FileVault"));
        assert!(engine.is_app_allowed("TasksGoals"));
        assert!(!engine.is_app_allowed("SystemMonitor"));
        assert!(!engine.is_app_allowed("Settings"));

        // WindDown
        engine.set_mode(OsFocusMode::WindDown);
        assert!(engine.is_app_allowed("TasksGoals"));
        assert!(engine.is_app_allowed("LayaAiChat"));
        assert!(!engine.is_app_allowed("StudyReader"));
        assert!(!engine.is_app_allowed("FileVault"));
        assert!(!engine.should_show_desktop_icons());
        assert!(!engine.should_show_left_widgets());
    }

    #[test]
    fn test_widget_column_toggles() {
        let mut engine = MindfulModeEngine::new();
        engine.set_mode(OsFocusMode::Mindful);
        assert!(engine.should_show_left_widgets());
        assert!(engine.should_show_desktop_icons());
        assert!(engine.should_show_search_bar());
        assert!(engine.should_show_bottom_dock());

        engine.toggle_left_widgets();
        assert!(!engine.should_show_left_widgets());

        engine.toggle_desktop_icons();
        assert!(!engine.should_show_desktop_icons());
    }
}
