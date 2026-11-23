//! Multi-Window Compositor for MindSet OS: Z-order stack, drag, resize, minimize, maximize, and tile snap.

use mindset_motion::{Spring1D, SpringPreset};
use mindset_ui::LayoutRect;

/// Standard identifiers for native MindSet OS application windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn all() -> [WindowId; 8] {
        [
            Self::Tasks,
            Self::StudyReader,
            Self::LayaAiChat,
            Self::SearchBrowser,
            Self::FileVault,
            Self::ProgressStats,
            Self::SystemMonitor,
            Self::Settings,
        ]
    }

    pub fn dock_windows() -> [WindowId; 9] {
        [
            Self::Today,
            Self::Tasks,
            Self::StudyReader,
            Self::LayaAiChat,
            Self::SearchBrowser,
            Self::FileVault,
            Self::ProgressStats,
            Self::SystemMonitor,
            Self::Settings,
        ]
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Today => "Today",
            Self::Tasks => "Tasks & Goals",
            Self::StudyReader => "Study Session Reader",
            Self::LayaAiChat => "Maindy AI Study Tutor",
            Self::ProgressStats => "Progress & Stats",
            Self::Settings => "Settings & Personalization",
            Self::SystemMonitor => "System Telemetry & Doctor",
            Self::FileVault => "Files & Study Vault",
            Self::SearchBrowser => "MindSet Browser — Sovereign Visual Desktop",
            Self::DesignSystem => "Design System Showcase",
        }
    }

    pub fn icon_symbol(self) -> &'static str {
        match self {
            Self::Today => "[TD]",
            Self::Tasks => "[T]",
            Self::StudyReader => "[S]",
            Self::LayaAiChat => "[AI]",
            Self::ProgressStats => "[ST]",
            Self::Settings => "[CFG]",
            Self::SystemMonitor => "[SYS]",
            Self::FileVault => "[DIR]",
            Self::SearchBrowser => "[SRC]",
            Self::DesignSystem => "[DS]",
        }
    }

    pub fn badge_color(self) -> [f32; 4] {
        match self {
            Self::Today => [0.961, 0.620, 0.043, 1.0], // Solar Amber
            Self::Tasks => [0.063, 0.725, 0.506, 1.0], // Emerald #10B981
            Self::StudyReader => [0.231, 0.510, 0.965, 1.0], // Royal Blue #3B82F6
            Self::LayaAiChat => [0.388, 0.400, 0.945, 1.0], // Vivid Indigo/Violet #6366F1
            Self::FileVault => [0.961, 0.620, 0.043, 1.0], // Warm Amber #F59E0B
            Self::ProgressStats => [0.024, 0.714, 0.831, 1.0], // Cyan Teal #06B6D4
            Self::SystemMonitor => [0.957, 0.247, 0.369, 1.0], // Coral Red #F43F5E
            Self::Settings => [0.392, 0.455, 0.545, 1.0], // Slate #64748B
            Self::SearchBrowser => [0.145, 0.388, 0.922, 1.0], // Indigo Blue #2563EB
            Self::DesignSystem => [0.55, 0.35, 0.85, 1.0], // Purple
        }
    }

    pub fn badge_symbol(self) -> &'static str {
        match self {
            Self::Today => "☀️",
            Self::Tasks => "✓",
            Self::StudyReader => "RD",
            Self::LayaAiChat => "✦",
            Self::FileVault => "DIR",
            Self::ProgressStats => "ST",
            Self::SystemMonitor => "SYS",
            Self::Settings => "SET",
            Self::SearchBrowser => "SRC",
            Self::DesignSystem => "DS",
        }
    }

    /// Concise 2-line wrapped desktop shortcut label (max 12 chars per line)
    pub fn desktop_label(self) -> &'static str {
        match self {
            Self::Today => "Today\nFocus",
            Self::Tasks => "Tasks &\nGoals",
            Self::StudyReader => "Study\nReader",
            Self::LayaAiChat => "Maindy AI\nTutor",
            Self::FileVault => "Files &\nVault",
            Self::ProgressStats => "Progress &\nStats",
            Self::SystemMonitor => "System\nMonitor",
            Self::Settings => "OS\nSettings",
            Self::SearchBrowser => "Search &\nShield",
            Self::DesignSystem => "Design\nSystem",
        }
    }

    pub fn default_rect(self, screen_w: f32, screen_h: f32) -> LayoutRect {
        match self {
            Self::StudyReader => {
                let w = (screen_w * 0.52).clamp(480.0, 720.0);
                let h = (screen_h - 120.0).clamp(420.0, 680.0);
                LayoutRect::new(260.0, 48.0, w, h)
            }
            Self::LayaAiChat => {
                let w = (screen_w * 0.38).clamp(380.0, 520.0);
                let h = (screen_h - 130.0).clamp(400.0, 660.0);
                LayoutRect::new(screen_w - w - 24.0, 56.0, w, h)
            }
            Self::Tasks => {
                let w = 460.0_f32.min(screen_w - 40.0);
                let h = 480.0_f32.min(screen_h - 100.0);
                LayoutRect::new(280.0, 64.0, w, h)
            }
            Self::ProgressStats => {
                let w = 520.0_f32.min(screen_w - 40.0);
                let h = 460.0_f32.min(screen_h - 100.0);
                LayoutRect::new(320.0, 72.0, w, h)
            }
            Self::Settings => {
                let w = 860.0_f32.min(screen_w - 40.0);
                let h = 600.0_f32.min(screen_h - 90.0);
                let x = ((screen_w - w) * 0.5).max(20.0);
                let y = ((screen_h - h) * 0.45).max(35.0);
                LayoutRect::new(x, y, w, h)
            }
            Self::SystemMonitor => {
                let w = 440.0_f32.min(screen_w - 40.0);
                let h = 500.0_f32.min(screen_h - 80.0);
                LayoutRect::new(300.0, 60.0, w, h)
            }
            Self::FileVault => {
                let w = 520.0_f32.min(screen_w - 40.0);
                let h = 480.0_f32.min(screen_h - 90.0);
                LayoutRect::new(340.0, 56.0, w, h)
            }
            Self::SearchBrowser => {
                let w = (screen_w * 0.75).clamp(850.0, 1250.0);
                let h = (screen_h - 100.0).clamp(580.0, 800.0);
                LayoutRect::new(((screen_w - w) * 0.5).max(20.0), 48.0, w, h)
            }
            Self::Today => {
                let top_h = crate::session::TOP_BAR_HEIGHT;
                let dock_h = crate::session::DOCK_HEIGHT;
                LayoutRect::new(0.0, top_h, screen_w, (screen_h - top_h - dock_h).max(100.0))
            }
            Self::DesignSystem => {
                let w = (screen_w * 0.70).clamp(700.0, 1100.0);
                let h = (screen_h - 100.0).clamp(500.0, 750.0);
                LayoutRect::new(100.0, 50.0, w, h)
            }
        }
    }
}

/// Dynamic tiling and layout state of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Floating,
    Maximized,
    SnappedLeft,
    SnappedRight,
    Minimized,
}

pub const TITLEBAR_HEIGHT: f32 = 36.0;
pub const MIN_WINDOW_WIDTH: f32 = 360.0;
pub const MIN_WINDOW_HEIGHT: f32 = 260.0;
pub const RESIZE_HANDLE_SIZE: f32 = 18.0;

/// A native surface window instance in the compositor.
#[derive(Debug, Clone)]
pub struct SurfaceWindow {
    pub id: WindowId,
    pub title: String,
    pub workspace_id: u8,
    pub rect: LayoutRect,
    pub saved_floating_rect: LayoutRect,
    pub state: WindowState,
    pub is_open: bool,
    pub z_index: usize,
    pub scroll_y: Spring1D,
    pub content_height: f32,
    pub is_scrolling_thumb: bool,

    pub is_dragging: bool,
    pub drag_offset: (f32, f32),
    pub is_resizing: bool,
    pub is_root: bool,
    pub has_controls: bool,
}

impl SurfaceWindow {
    pub fn new(id: WindowId, initial_rect: LayoutRect, workspace_id: u8) -> Self {
        Self {
            id,
            title: id.title().to_string(),
            workspace_id,
            rect: initial_rect,
            saved_floating_rect: initial_rect,
            state: WindowState::Floating,
            is_open: true,
            z_index: 0,
            scroll_y: Spring1D::with_preset(0.0, SpringPreset::FluidPanel),
            content_height: initial_rect.h,
            is_scrolling_thumb: false,
            is_dragging: false,
            drag_offset: (0.0, 0.0),
            is_resizing: false,
            is_root: false,
            has_controls: true,
        }
    }

    pub fn today(screen_w: f32, screen_h: f32) -> Self {
        let top_h = crate::session::TOP_BAR_HEIGHT;
        let dock_h = crate::session::DOCK_HEIGHT;
        let r = LayoutRect::new(0.0, top_h, screen_w, (screen_h - top_h - dock_h).max(100.0));
        Self {
            id: WindowId::Today,
            title: "Today".to_string(),
            workspace_id: 0,
            rect: r,
            saved_floating_rect: r,
            state: WindowState::Floating,
            is_open: true,
            z_index: 0,
            scroll_y: Spring1D::with_preset(0.0, SpringPreset::FluidPanel),
            content_height: r.h,
            is_scrolling_thumb: false,
            is_dragging: false,
            drag_offset: (0.0, 0.0),
            is_resizing: false,
            is_root: true,
            has_controls: false,
        }
    }

    pub fn scrollbar_track_rect(&self) -> LayoutRect {
        let content = self.content_rect();
        LayoutRect::new(
            content.x + content.w - 10.0,
            content.y + 4.0,
            6.0,
            (content.h - 8.0).max(10.0),
        )
    }

    pub fn scrollbar_thumb_rect(&self) -> Option<LayoutRect> {
        let content = self.content_rect();
        let track = self.scrollbar_track_rect();
        if self.content_height <= content.h || content.h <= 0.0 {
            return None;
        }
        let ratio = (content.h / self.content_height).clamp(0.12, 0.88);
        let thumb_h = (track.h * ratio).max(28.0);
        let max_scroll = (self.content_height - content.h).max(1.0);
        let scroll_ratio = (self.scroll_y.position / max_scroll).clamp(0.0, 1.0);
        let thumb_y = track.y + scroll_ratio * (track.h - thumb_h);
        Some(LayoutRect::new(track.x, thumb_y, track.w, thumb_h))
    }

    pub fn scroll_by(&mut self, dy: f32) {
        let content = self.content_rect();
        let max_scroll = (self.content_height - content.h).max(0.0);
        let target = (self.scroll_y.target + dy).clamp(0.0, max_scroll);
        self.scroll_y.set_target(target);
    }

    pub fn titlebar_rect(&self) -> LayoutRect {
        LayoutRect::new(self.rect.x, self.rect.y, self.rect.w, TITLEBAR_HEIGHT)
    }

    pub fn content_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x,
            self.rect.y + TITLEBAR_HEIGHT,
            self.rect.w,
            (self.rect.h - TITLEBAR_HEIGHT).max(0.0),
        )
    }

    pub fn chrome_targets(&self, ui_scale: f32) -> crate::layout::WindowChromeHitTargets {
        crate::layout::WindowChromeHitTargets::from_rect(self.rect, ui_scale)
    }

    pub fn close_button_rect(&self) -> LayoutRect {
        self.chrome_targets(1.0).close_btn
    }

    pub fn close_button_rect_scaled(&self, ui_scale: f32) -> LayoutRect {
        self.chrome_targets(ui_scale).close_btn
    }

    pub fn maximize_button_rect(&self) -> LayoutRect {
        self.chrome_targets(1.0).max_btn
    }

    pub fn maximize_button_rect_scaled(&self, ui_scale: f32) -> LayoutRect {
        self.chrome_targets(ui_scale).max_btn
    }

    pub fn minimize_button_rect(&self) -> LayoutRect {
        self.chrome_targets(1.0).min_btn
    }

    pub fn minimize_button_rect_scaled(&self, ui_scale: f32) -> LayoutRect {
        self.chrome_targets(ui_scale).min_btn
    }

    pub fn resize_grip_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x + self.rect.w - RESIZE_HANDLE_SIZE,
            self.rect.y + self.rect.h - RESIZE_HANDLE_SIZE,
            RESIZE_HANDLE_SIZE,
            RESIZE_HANDLE_SIZE,
        )
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        self.is_open && self.state != WindowState::Minimized && self.rect.contains(px, py)
    }
}

/// The multi-window compositor managing Z-order stacking, tiling, and window state.
pub struct WindowManager {
    pub windows: Vec<SurfaceWindow>,
    pub active_window_id: Option<WindowId>,
    pub screen_width: f32,
    pub screen_height: f32,
    pub top_bar_height: f32,
    pub dock_height: f32,
    next_z: usize,
}

impl WindowManager {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut mgr = Self {
            windows: Vec::new(),
            active_window_id: None,
            screen_width,
            screen_height,
            top_bar_height: 44.0,
            dock_height: 56.0,
            next_z: 1,
        };

        // Seed root Today window
        let mut today_win = SurfaceWindow::today(screen_width, screen_height);
        today_win.is_open = true;
        today_win.z_index = 0;
        mgr.windows.push(today_win);

        // Seed standard application windows in default closed state
        for id in WindowId::all() {
            let r = id.default_rect(screen_width, screen_height);
            let mut win = SurfaceWindow::new(id, r, 0);
            win.is_open = false;
            mgr.windows.push(win);
        }

        // Seed DesignSystem showcase window (closed)
        let ds_r = WindowId::DesignSystem.default_rect(screen_width, screen_height);
        let mut ds_win = SurfaceWindow::new(WindowId::DesignSystem, ds_r, 0);
        ds_win.is_open = false;
        mgr.windows.push(ds_win);

        mgr.active_window_id = Some(WindowId::Today);
        mgr
    }

    pub fn resize_viewport(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        let usable = self.usable_desktop_rect();

        for win in &mut self.windows {
            if win.state == WindowState::Maximized {
                win.rect = usable;
            } else if win.state == WindowState::SnappedLeft {
                win.rect = LayoutRect::new(usable.x, usable.y, usable.w * 0.5, usable.h);
            } else if win.state == WindowState::SnappedRight {
                win.rect = LayoutRect::new(
                    usable.x + usable.w * 0.5,
                    usable.y,
                    usable.w * 0.5,
                    usable.h,
                );
            }
        }
    }

    pub fn usable_desktop_rect(&self) -> LayoutRect {
        LayoutRect::new(
            0.0,
            self.top_bar_height,
            self.screen_width,
            (self.screen_height - self.top_bar_height - self.dock_height).max(100.0),
        )
    }

    pub fn find_window_mut(&mut self, id: WindowId) -> Option<&mut SurfaceWindow> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn find_window(&self, id: WindowId) -> Option<&SurfaceWindow> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Brings a window to the top of the Z-stack and sets it as the active window.
    pub fn promote_to_front(&mut self, id: WindowId) {
        if id == WindowId::Today {
            self.active_window_id = Some(WindowId::Today);
            return;
        }
        self.next_z += 1;
        let z = self.next_z;

        if let Some(win) = self.find_window_mut(id) {
            win.is_open = true;
            if win.state == WindowState::Minimized {
                win.state = WindowState::Floating;
            }
            win.z_index = z;
            self.active_window_id = Some(id);
        }
    }

    /// Toggles window open/closed or restores from minimized state.
    pub fn toggle_window(&mut self, id: WindowId) {
        if id == WindowId::Today {
            self.active_window_id = Some(WindowId::Today);
            return;
        }
        let is_currently_open = self
            .find_window(id)
            .is_some_and(|w| w.is_open && w.state != WindowState::Minimized);
        if is_currently_open && self.active_window_id == Some(id) {
            self.minimize_window(id);
        } else {
            self.promote_to_front(id);
        }
    }

    pub fn minimize_window(&mut self, id: WindowId) {
        if id == WindowId::Today {
            return; // Root Today window is never minimizable
        }
        if let Some(win) = self.find_window_mut(id) {
            win.state = WindowState::Minimized;
        }

        // Shift active focus to the next highest z-index open window (falling back to Today)
        let next_active = self
            .windows
            .iter()
            .filter(|w| w.is_open && w.state != WindowState::Minimized && !w.is_root)
            .max_by_key(|w| w.z_index)
            .map(|w| w.id)
            .or(Some(WindowId::Today));

        self.active_window_id = next_active;
    }

    pub fn close_window(&mut self, id: WindowId) {
        if id == WindowId::Today {
            return; // Root Today window is never closable
        }
        if let Some(win) = self.find_window_mut(id) {
            win.is_open = false;
        }

        if self.active_window_id == Some(id) {
            self.active_window_id = self
                .windows
                .iter()
                .filter(|w| w.is_open && w.state != WindowState::Minimized && !w.is_root)
                .max_by_key(|w| w.z_index)
                .map(|w| w.id)
                .or(Some(WindowId::Today));
        }
    }

    pub fn toggle_maximize(&mut self, id: WindowId) {
        let usable = self.usable_desktop_rect();
        if let Some(win) = self.find_window_mut(id) {
            match win.state {
                WindowState::Maximized => {
                    win.rect = win.saved_floating_rect;
                    win.state = WindowState::Floating;
                }
                _ => {
                    win.saved_floating_rect = win.rect;
                    win.rect = usable;
                    win.state = WindowState::Maximized;
                }
            }
        }
    }

    pub fn snap_window(&mut self, id: WindowId, state: WindowState) {
        let usable = self.usable_desktop_rect();
        if let Some(win) = self.find_window_mut(id) {
            if win.state == WindowState::Floating {
                win.saved_floating_rect = win.rect;
            }
            match state {
                WindowState::SnappedLeft => {
                    win.rect = LayoutRect::new(usable.x, usable.y, usable.w * 0.5, usable.h);
                    win.state = WindowState::SnappedLeft;
                }
                WindowState::SnappedRight => {
                    win.rect = LayoutRect::new(
                        usable.x + usable.w * 0.5,
                        usable.y,
                        usable.w * 0.5,
                        usable.h,
                    );
                    win.state = WindowState::SnappedRight;
                }
                WindowState::Maximized => {
                    win.rect = usable;
                    win.state = WindowState::Maximized;
                }
                WindowState::Floating => {
                    win.rect = win.saved_floating_rect;
                    win.state = WindowState::Floating;
                }
                WindowState::Minimized => {
                    win.state = WindowState::Minimized;
                }
            }
        }
    }

    /// Evaluates edge snapping when dragging a window stops.
    pub fn release_drag(&mut self, id: WindowId, cursor_x: f32, cursor_y: f32) {
        if let Some(win) = self.find_window_mut(id) {
            win.is_dragging = false;
        }

        if cursor_x < 24.0 {
            self.snap_window(id, WindowState::SnappedLeft);
        } else if cursor_x > self.screen_width - 24.0 {
            self.snap_window(id, WindowState::SnappedRight);
        } else if cursor_y < self.top_bar_height + 12.0 {
            self.snap_window(id, WindowState::Maximized);
        }
    }

    /// Returns references to all visible windows sorted back-to-front by Z-index.
    pub fn sorted_visible_windows(&self, current_workspace: u8) -> Vec<&SurfaceWindow> {
        let mut list: Vec<&SurfaceWindow> = self
            .windows
            .iter()
            .filter(|w| {
                w.is_open
                    && w.state != WindowState::Minimized
                    && w.workspace_id == current_workspace
            })
            .collect();
        list.sort_by_key(|w| w.z_index);
        list
    }

    /// Returns all floating application windows for Alt+Tab cycling and [Windows: N Open] count.
    /// Explicitly excludes root Today window and DesignSystem showcase.
    pub fn open_app_windows(&self, current_workspace: u8) -> Vec<&SurfaceWindow> {
        self.windows
            .iter()
            .filter(|w| {
                w.is_open
                    && w.state != WindowState::Minimized
                    && w.workspace_id == current_workspace
                    && !w.is_root
                    && w.id != WindowId::DesignSystem
            })
            .collect()
    }

    /// Finds the top-most window (highest Z) under the cursor.
    pub fn window_at_point(&self, px: f32, py: f32, current_workspace: u8) -> Option<WindowId> {
        self.windows
            .iter()
            .filter(|w| !w.is_root && w.workspace_id == current_workspace && w.contains(px, py))
            .max_by_key(|w| w.z_index)
            .map(|w| w.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_win(wm: &WindowManager, id: WindowId) -> &SurfaceWindow {
        match wm.find_window(id) {
            Some(w) => w,
            None => panic!("window {id:?} not found in test"),
        }
    }

    fn test_win_mut(wm: &mut WindowManager, id: WindowId) -> &mut SurfaceWindow {
        match wm.find_window_mut(id) {
            Some(w) => w,
            None => panic!("window {id:?} not found in test"),
        }
    }

    #[test]
    fn test_window_z_order_promotion() {
        let mut wm = WindowManager::new(1280.0, 800.0);
        wm.promote_to_front(WindowId::Tasks);
        let z1 = test_win(&wm, WindowId::Tasks).z_index;

        wm.promote_to_front(WindowId::StudyReader);
        let z2 = test_win(&wm, WindowId::StudyReader).z_index;
        assert!(z2 > z1, "StudyReader should have higher Z than Tasks");
        assert_eq!(wm.active_window_id, Some(WindowId::StudyReader));

        // Click background Tasks window
        wm.promote_to_front(WindowId::Tasks);
        let z3 = test_win(&wm, WindowId::Tasks).z_index;
        assert!(
            z3 > z2,
            "Promoting Tasks must raise its Z-order above StudyReader"
        );
        assert_eq!(wm.active_window_id, Some(WindowId::Tasks));
    }

    #[test]
    fn test_window_edge_snap_state_transitions() {
        let mut wm = WindowManager::new(1280.0, 800.0);
        let initial_rect = test_win(&wm, WindowId::Tasks).rect;

        // Snap left
        wm.snap_window(WindowId::Tasks, WindowState::SnappedLeft);
        let snapped_l = test_win(&wm, WindowId::Tasks);
        assert_eq!(snapped_l.state, WindowState::SnappedLeft);
        assert_eq!(snapped_l.rect.x, 0.0);
        assert_eq!(snapped_l.rect.w, 1280.0 * 0.5);

        // Restore floating
        wm.snap_window(WindowId::Tasks, WindowState::Floating);
        let restored = test_win(&wm, WindowId::Tasks);
        assert_eq!(restored.state, WindowState::Floating);
        assert_eq!(restored.rect.x, initial_rect.x);
        assert_eq!(restored.rect.w, initial_rect.w);

        // Maximize and restore
        wm.toggle_maximize(WindowId::Tasks);
        assert_eq!(test_win(&wm, WindowId::Tasks).state, WindowState::Maximized);
        wm.toggle_maximize(WindowId::Tasks);
        assert_eq!(test_win(&wm, WindowId::Tasks).state, WindowState::Floating);
    }

    #[test]
    fn test_window_minimum_resize_clamp() {
        let mut wm = WindowManager::new(1280.0, 800.0);
        let win = test_win_mut(&mut wm, WindowId::Tasks);

        // Drag resize grip to tiny dimensions
        let new_w = 100.0_f32.max(MIN_WINDOW_WIDTH);
        let new_h = 100.0_f32.max(MIN_WINDOW_HEIGHT);
        win.rect.w = new_w;
        win.rect.h = new_h;

        assert_eq!(win.rect.w, 360.0);
        assert_eq!(win.rect.h, 260.0);
    }

    #[test]
    fn test_today_is_only_launch_window() {
        let wm = WindowManager::new(1280.0, 800.0);
        let open_wins: Vec<_> = wm.windows.iter().filter(|w| w.is_open).collect();
        assert_eq!(
            open_wins.len(),
            1,
            "Exactly one window should be open on launch"
        );
        assert_eq!(open_wins[0].id, WindowId::Today);
        assert!(open_wins[0].is_root);
        assert!(!open_wins[0].has_controls);
    }

    #[test]
    fn test_today_not_in_alt_tab_cycle() {
        let mut wm = WindowManager::new(1280.0, 800.0);
        // Initially only Today is open
        let cycle1 = wm.open_app_windows(0);
        assert_eq!(
            cycle1.len(),
            0,
            "Today must never be included in Alt+Tab window cycling"
        );

        // Open Tasks and StudyReader
        wm.promote_to_front(WindowId::Tasks);
        wm.promote_to_front(WindowId::StudyReader);
        let cycle2 = wm.open_app_windows(0);
        assert_eq!(cycle2.len(), 2);
        assert!(cycle2
            .iter()
            .all(|w| w.id != WindowId::Today && w.id != WindowId::DesignSystem));
    }
}
