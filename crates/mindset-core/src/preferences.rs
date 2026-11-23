//! System preferences, theme models, global scales, and toast notifications for MindSet OS.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    AlpineGlass,
    DarkSpace,
    VioletNebula,
    SolarAmber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PreferencesTab {
    #[default]
    Appearance,
    FocusShields,
    AudioRadios,
    LanguageRegion,
    MaindyAiKeys,
    HardwarePower,
}

impl PreferencesTab {
    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => Self::Appearance,
            1 => Self::FocusShields,
            2 => Self::AudioRadios,
            3 => Self::LanguageRegion,
            4 => Self::MaindyAiKeys,
            5 => Self::HardwarePower,
            _ => Self::Appearance,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Self::Appearance => 0,
            Self::FocusShields => 1,
            Self::AudioRadios => 2,
            Self::LanguageRegion => 3,
            Self::MaindyAiKeys => 4,
            Self::HardwarePower => 5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Appearance => "🎨 Appearance",
            Self::FocusShields => "🎯 Focus & Shields",
            Self::AudioRadios => "🔊 Audio & Radios",
            Self::LanguageRegion => "🌐 Language & Region",
            Self::MaindyAiKeys => "🧠 Maindy AI & Keys",
            Self::HardwarePower => "💻 Hardware & Power",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPreferences {
    pub theme: ThemeMode,
    pub ui_scale: f32,
    pub brightness: f32,
    pub volume: f32,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub focus_dnd: bool,
    pub show_desktop: bool,
    #[serde(default)]
    pub active_tab: PreferencesTab,
    #[serde(default)]
    pub airplane_mode: bool,
    #[serde(default = "default_true")]
    pub prev_wifi: bool,
    #[serde(default = "default_true")]
    pub prev_bluetooth: bool,
}

fn default_true() -> bool {
    true
}

impl Default for SystemPreferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::AlpineGlass,
            ui_scale: 1.0,
            brightness: 1.0,
            volume: 0.8,
            wifi_enabled: true,
            bluetooth_enabled: true,
            focus_dnd: false,
            show_desktop: true,
            active_tab: PreferencesTab::Appearance,
            airplane_mode: false,
            prev_wifi: true,
            prev_bluetooth: true,
        }
    }
}

impl SystemPreferences {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_active_tab(&mut self, tab: PreferencesTab) {
        self.active_tab = tab;
    }

    pub fn set_active_tab_index(&mut self, idx: usize) {
        self.active_tab = PreferencesTab::from_index(idx);
    }

    pub fn set_ui_scale(&mut self, scale: f32) {
        // Enforce strict scaling clamp between 0.80 and 1.45
        self.ui_scale = scale.clamp(0.80, 1.45);
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.20, 1.0);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle_wifi(&mut self) {
        if !self.airplane_mode {
            self.wifi_enabled = !self.wifi_enabled;
        }
    }

    pub fn toggle_bluetooth(&mut self) {
        if !self.airplane_mode {
            self.bluetooth_enabled = !self.bluetooth_enabled;
        }
    }

    pub fn toggle_airplane_mode(&mut self) {
        if !self.airplane_mode {
            // Turning ON: save previous states and disable radios
            self.prev_wifi = self.wifi_enabled;
            self.prev_bluetooth = self.bluetooth_enabled;
            self.wifi_enabled = false;
            self.bluetooth_enabled = false;
            self.airplane_mode = true;
        } else {
            // Turning OFF: restore prior states
            self.wifi_enabled = self.prev_wifi;
            self.bluetooth_enabled = self.prev_bluetooth;
            self.airplane_mode = false;
        }
    }

    pub fn toggle_dnd(&mut self) {
        self.focus_dnd = !self.focus_dnd;
    }

    pub fn toggle_desktop(&mut self) {
        self.show_desktop = !self.show_desktop;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastMessage {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub icon: String,
    pub remaining_secs: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToastQueue {
    pub toasts: Vec<ToastMessage>,
    pub next_id: u64,
}

impl ToastQueue {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 1,
        }
    }

    pub fn push(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
        icon: impl Into<String>,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(ToastMessage {
            id,
            title: title.into(),
            body: body.into(),
            icon: icon.into(),
            remaining_secs: 4.0,
        });
    }

    pub fn tick(&mut self, dt: f32) {
        for toast in self.toasts.iter_mut() {
            toast.remaining_secs -= dt;
        }
        self.toasts.retain(|t| t.remaining_secs > 0.0);
    }

    pub fn dismiss(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_clamping() {
        let mut prefs = SystemPreferences::new();
        prefs.set_ui_scale(0.5);
        assert!((prefs.ui_scale - 0.80).abs() < f32::EPSILON);

        prefs.set_ui_scale(2.0);
        assert!((prefs.ui_scale - 1.45).abs() < f32::EPSILON);

        prefs.set_ui_scale(1.15);
        assert!((prefs.ui_scale - 1.15).abs() < f32::EPSILON);
    }

    #[test]
    fn test_airplane_mode_interlock() {
        let mut prefs = SystemPreferences::new();
        assert!(prefs.wifi_enabled);
        assert!(prefs.bluetooth_enabled);
        assert!(!prefs.airplane_mode);

        // Turn on Airplane mode: radios disable, state preserved
        prefs.toggle_airplane_mode();
        assert!(prefs.airplane_mode);
        assert!(!prefs.wifi_enabled);
        assert!(!prefs.bluetooth_enabled);

        // While in airplane mode, direct toggles are locked
        prefs.toggle_wifi();
        assert!(!prefs.wifi_enabled);
        prefs.toggle_bluetooth();
        assert!(!prefs.bluetooth_enabled);

        // Turn off Airplane mode: radios restore to previous states
        prefs.toggle_airplane_mode();
        assert!(!prefs.airplane_mode);
        assert!(prefs.wifi_enabled);
        assert!(prefs.bluetooth_enabled);
    }

    #[test]
    fn test_toast_queue_lifecycle() {
        let mut queue = ToastQueue::new();
        queue.push("Focus Started", "25m deep study session initiated", "🎯");
        assert_eq!(queue.toasts.len(), 1);

        queue.tick(2.0);
        assert_eq!(queue.toasts.len(), 1);

        queue.tick(2.5);
        assert_eq!(queue.toasts.len(), 0);
    }
}
