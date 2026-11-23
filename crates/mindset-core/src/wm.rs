//! MindSet OS Window Management and Compile-Time Modal Routing.
//!
//! Provides pure-domain models for window states, z-stack ordering,
//! bounds, and mutually exclusive overlays.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowId {
    Today,
    Tasks,
    StudyReader,
    LayaAiChat,
    ProgressStats,
    Settings,
    SystemMonitor,
    FileVault,
    SearchBrowser,
    DesignSystem,
}

impl WindowId {
    pub fn default_title(&self) -> &'static str {
        match self {
            WindowId::Today => "Today — Deep Focus Dashboard",
            WindowId::Tasks => "Syllabus Quotas & Goals",
            WindowId::StudyReader => "Study Session Reader",
            WindowId::LayaAiChat => "Maindy AI Tutor",
            WindowId::ProgressStats => "Progress & Analytics",
            WindowId::Settings => "MindSet Preferences",
            WindowId::SystemMonitor => "System Resource Monitor",
            WindowId::FileVault => "Student File Vault",
            WindowId::SearchBrowser => "MindStrict Safe Browser",
            WindowId::DesignSystem => "Liquidmorphism Design System",
        }
    }

    pub fn default_icon(&self) -> &'static str {
        match self {
            WindowId::Today => "☀️",
            WindowId::Tasks => "📋",
            WindowId::StudyReader => "📖",
            WindowId::LayaAiChat => "🧠",
            WindowId::ProgressStats => "📊",
            WindowId::Settings => "⚙️",
            WindowId::SystemMonitor => "💻",
            WindowId::FileVault => "📁",
            WindowId::SearchBrowser => "🌐",
            WindowId::DesignSystem => "🎨",
        }
    }

    pub fn default_dimensions(&self) -> (f32, f32) {
        match self {
            WindowId::Today => (840.0, 580.0),
            WindowId::Tasks => (780.0, 520.0),
            WindowId::StudyReader => (920.0, 620.0),
            WindowId::LayaAiChat => (760.0, 540.0),
            WindowId::ProgressStats => (800.0, 520.0),
            WindowId::Settings => (720.0, 480.0),
            WindowId::SystemMonitor => (680.0, 440.0),
            WindowId::FileVault => (740.0, 480.0),
            WindowId::SearchBrowser => (960.0, 600.0),
            WindowId::DesignSystem => (800.0, 540.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowMode {
    Normal,
    Maximized,
    Minimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInstance {
    pub id: WindowId,
    pub title: String,
    pub icon: String,
    pub is_open: bool,
    pub mode: WindowMode,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub pre_max_x: f32,
    pub pre_max_y: f32,
    pub pre_max_width: f32,
    pub pre_max_height: f32,
}

impl WindowInstance {
    pub fn new(id: WindowId, initial_x: f32, initial_y: f32) -> Self {
        let (w, h) = id.default_dimensions();
        Self {
            id,
            title: id.default_title().to_string(),
            icon: id.default_icon().to_string(),
            is_open: false,
            mode: WindowMode::Normal,
            x: initial_x,
            y: initial_y,
            width: w,
            height: h,
            pre_max_x: initial_x,
            pre_max_y: initial_y,
            pre_max_width: w,
            pre_max_height: h,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ActiveOverlay {
    #[default]
    None,
    Launcher,
    ControlCenter,
    OdysseyMap,
    MindfulGuard,
    QuickSetupWizard,
    Notifications,
    PowerMenu,
    ExamReport,
    VerificationBoss,
    ExcuseRoast,
    BreakArcade,
    LockScreen,
    WindowSwitcher,
    AgentPermission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesktopWidgetId {
    ScholarBriefing,
    OdysseyTrail,
    Scratchpad,
}

impl DesktopWidgetId {
    pub fn title(&self) -> &'static str {
        match self {
            DesktopWidgetId::ScholarBriefing => "Daily AI Briefing",
            DesktopWidgetId::OdysseyTrail => "3D Syllabus Odyssey Trail",
            DesktopWidgetId::Scratchpad => "Quick Study Scratchpad",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesktopWidgetPlacement {
    pub id: DesktopWidgetId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_visible: bool,
    pub is_pinned: bool,
}

impl DesktopWidgetPlacement {
    pub fn new(id: DesktopWidgetId, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            is_visible: true,
            is_pinned: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesktopLayoutConfig {
    pub free_placement: bool,
    pub widget_order: Vec<DesktopWidgetId>,
    pub placements: Vec<DesktopWidgetPlacement>,
}

impl Default for DesktopLayoutConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopLayoutConfig {
    pub fn new() -> Self {
        Self {
            free_placement: false,
            widget_order: vec![
                DesktopWidgetId::ScholarBriefing,
                DesktopWidgetId::OdysseyTrail,
                DesktopWidgetId::Scratchpad,
            ],
            placements: vec![
                DesktopWidgetPlacement::new(DesktopWidgetId::ScholarBriefing, 410.0, 16.0, 460.0, 100.0),
                DesktopWidgetPlacement::new(DesktopWidgetId::OdysseyTrail, 410.0, 130.0, 460.0, 310.0),
                DesktopWidgetPlacement::new(DesktopWidgetId::Scratchpad, 410.0, 455.0, 460.0, 140.0),
            ],
        }
    }

    pub fn get_placement(&self, id: DesktopWidgetId) -> Option<&DesktopWidgetPlacement> {
        self.placements.iter().find(|p| p.id == id)
    }

    pub fn get_placement_mut(&mut self, id: DesktopWidgetId) -> Option<&mut DesktopWidgetPlacement> {
        self.placements.iter_mut().find(|p| p.id == id)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowManager {
    pub windows: Vec<WindowInstance>,
    pub z_stack: Vec<WindowId>,
    pub active_overlay: ActiveOverlay,
    pub widget_layout: DesktopLayoutConfig,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        let all_ids = [
            WindowId::Today,
            WindowId::Tasks,
            WindowId::StudyReader,
            WindowId::LayaAiChat,
            WindowId::ProgressStats,
            WindowId::Settings,
            WindowId::SystemMonitor,
            WindowId::FileVault,
            WindowId::SearchBrowser,
            WindowId::DesignSystem,
        ];

        let mut windows = Vec::new();
        let mut x = 80.0;
        let mut y = 60.0;

        for id in all_ids {
            windows.push(WindowInstance::new(id, x, y));
            x += 30.0;
            y += 24.0;
        }

        // Today is open by default as launch view
        if let Some(w) = windows.iter_mut().find(|w| w.id == WindowId::Today) {
            w.is_open = true;
        }

        Self {
            windows,
            z_stack: vec![WindowId::Today],
            active_overlay: ActiveOverlay::None,
            widget_layout: DesktopLayoutConfig::new(),
        }
    }

    pub fn get_widget_placement(&self, id: DesktopWidgetId) -> Option<&DesktopWidgetPlacement> {
        self.widget_layout.get_placement(id)
    }

    pub fn get_widget_placement_mut(&mut self, id: DesktopWidgetId) -> Option<&mut DesktopWidgetPlacement> {
        self.widget_layout.get_placement_mut(id)
    }

    pub fn get_window(&self, id: WindowId) -> Option<&WindowInstance> {
        self.windows.iter().find(|w| w.id == id)
    }

    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut WindowInstance> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn active_window(&self) -> Option<WindowId> {
        self.z_stack.last().copied()
    }

    pub fn is_open(&self, id: WindowId) -> bool {
        self.get_window(id).is_some_and(|w| w.is_open)
    }

    pub fn open(&mut self, id: WindowId) {
        if let Some(win) = self.get_window_mut(id) {
            win.is_open = true;
            if win.mode == WindowMode::Minimized {
                win.mode = WindowMode::Normal;
            }
        }
        self.bring_to_front(id);
    }

    pub fn close(&mut self, id: WindowId) {
        if let Some(win) = self.get_window_mut(id) {
            win.is_open = false;
        }
        self.z_stack.retain(|&wid| wid != id);
    }

    pub fn minimize(&mut self, id: WindowId) {
        if let Some(win) = self.get_window_mut(id) {
            win.mode = WindowMode::Minimized;
        }
        self.z_stack.retain(|&wid| wid != id);
    }

    pub fn toggle_maximize(&mut self, id: WindowId, screen_w: f32, screen_h: f32) {
        if let Some(win) = self.get_window_mut(id) {
            match win.mode {
                WindowMode::Normal => {
                    win.pre_max_x = win.x;
                    win.pre_max_y = win.y;
                    win.pre_max_width = win.width;
                    win.pre_max_height = win.height;

                    win.x = 0.0;
                    win.y = 38.0; // below top bar
                    win.width = screen_w;
                    win.height = (screen_h - 38.0 - 64.0).max(200.0); // above dock
                    win.mode = WindowMode::Maximized;
                }
                WindowMode::Maximized => {
                    win.x = win.pre_max_x;
                    win.y = win.pre_max_y;
                    win.width = win.pre_max_width;
                    win.height = win.pre_max_height;
                    win.mode = WindowMode::Normal;
                }
                WindowMode::Minimized => {
                    win.mode = WindowMode::Normal;
                }
            }
        }
    }

    pub fn bring_to_front(&mut self, id: WindowId) {
        self.z_stack.retain(|&wid| wid != id);
        self.z_stack.push(id);
    }

    pub fn toggle_overlay(&mut self, overlay: ActiveOverlay) {
        if self.active_overlay == overlay {
            self.active_overlay = ActiveOverlay::None;
        } else {
            self.active_overlay = overlay;
        }
    }

    pub fn close_overlays(&mut self) {
        self.active_overlay = ActiveOverlay::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wm_initialization() {
        let wm = WindowManager::new();
        assert!(wm.is_open(WindowId::Today));
        assert_eq!(wm.active_window(), Some(WindowId::Today));
        assert_eq!(wm.active_overlay, ActiveOverlay::None);
    }

    #[test]
    fn test_wm_open_close_cycle() {
        let mut wm = WindowManager::new();
        wm.open(WindowId::StudyReader);
        assert!(wm.is_open(WindowId::StudyReader));
        assert_eq!(wm.active_window(), Some(WindowId::StudyReader));

        wm.open(WindowId::Tasks);
        assert_eq!(wm.active_window(), Some(WindowId::Tasks));

        wm.close(WindowId::Tasks);
        assert!(!wm.is_open(WindowId::Tasks));
        assert_eq!(wm.active_window(), Some(WindowId::StudyReader));
    }

    #[test]
    fn test_wm_maximize_restore() {
        let mut wm = WindowManager::new();
        let (orig_w, orig_h) = WindowId::StudyReader.default_dimensions();

        wm.open(WindowId::StudyReader);
        wm.toggle_maximize(WindowId::StudyReader, 1920.0, 1080.0);

        let win = wm.get_window(WindowId::StudyReader).unwrap();
        assert_eq!(win.mode, WindowMode::Maximized);
        assert_eq!(win.width, 1920.0);

        wm.toggle_maximize(WindowId::StudyReader, 1920.0, 1080.0);
        let win = wm.get_window(WindowId::StudyReader).unwrap();
        assert_eq!(win.mode, WindowMode::Normal);
        assert_eq!(win.width, orig_w);
        assert_eq!(win.height, orig_h);
    }

    #[test]
    fn test_overlay_toggle() {
        let mut wm = WindowManager::new();
        wm.toggle_overlay(ActiveOverlay::ControlCenter);
        assert_eq!(wm.active_overlay, ActiveOverlay::ControlCenter);

        wm.toggle_overlay(ActiveOverlay::ControlCenter);
        assert_eq!(wm.active_overlay, ActiveOverlay::None);

        wm.toggle_overlay(ActiveOverlay::Launcher);
        assert_eq!(wm.active_overlay, ActiveOverlay::Launcher);
        wm.close_overlays();
        assert_eq!(wm.active_overlay, ActiveOverlay::None);
    }

    #[test]
    fn test_bring_to_front_cleanly() {
        let mut wm = WindowManager::new();
        wm.open(WindowId::StudyReader);
        wm.open(WindowId::FileVault);
        wm.open(WindowId::Settings);

        assert_eq!(wm.active_window(), Some(WindowId::Settings));
        assert_eq!(
            wm.z_stack,
            vec![
                WindowId::Today,
                WindowId::StudyReader,
                WindowId::FileVault,
                WindowId::Settings
            ]
        );

        // Click StudyReader to bring to front
        wm.bring_to_front(WindowId::StudyReader);
        assert_eq!(wm.active_window(), Some(WindowId::StudyReader));
        assert_eq!(
            wm.z_stack,
            vec![
                WindowId::Today,
                WindowId::FileVault,
                WindowId::Settings,
                WindowId::StudyReader
            ]
        );
    }

    #[test]
    fn test_desktop_widget_placement_and_reset() {
        let mut wm = WindowManager::new();
        assert_eq!(wm.widget_layout.widget_order.len(), 3);
        assert_eq!(wm.widget_layout.placements.len(), 3);

        let briefing = wm
            .get_widget_placement(DesktopWidgetId::ScholarBriefing)
            .unwrap();
        assert_eq!(briefing.x, 410.0);
        assert_eq!(briefing.y, 16.0);

        if let Some(p) = wm.get_widget_placement_mut(DesktopWidgetId::ScholarBriefing) {
            p.x = 100.0;
            p.y = 80.0;
        }

        assert_eq!(
            wm.get_widget_placement(DesktopWidgetId::ScholarBriefing)
                .unwrap()
                .x,
            100.0
        );

        wm.widget_layout.reset();
        assert_eq!(
            wm.get_widget_placement(DesktopWidgetId::ScholarBriefing)
                .unwrap()
                .x,
            410.0
        );
    }
}
