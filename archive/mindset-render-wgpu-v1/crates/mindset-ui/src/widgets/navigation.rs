//! Navigation, Sidebar NavigationRail, ModalDialog, and Toast notifications.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::{ElementId, WidgetState};
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};

/// Tab item metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabItem {
    pub id: usize,
    pub title: String,
    pub badge: Option<String>,
    pub icon_symbol: String,
}

/// Tabs controller widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabs {
    pub id: ElementId,
    pub items: Vec<TabItem>,
    pub active_idx: usize,
    pub state: WidgetState,
}

impl Tabs {
    pub fn new(id: ElementId, items: Vec<TabItem>, active_idx: usize) -> Self {
        Self {
            id,
            items,
            active_idx,
            state: WidgetState::Normal,
        }
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.active_idx = idx;
        }
    }

    pub fn active_item(&self) -> Option<&TabItem> {
        self.items.get(self.active_idx)
    }
}

/// NavigationRail sidebar widget for settings, personalizer, and studio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationRail {
    pub id: ElementId,
    pub tabs: Vec<TabItem>,
    pub active_tab: usize,
    pub width: f32,
    pub state: WidgetState,
}

impl NavigationRail {
    pub fn new(id: ElementId, tabs: Vec<TabItem>, active_tab: usize, width: f32) -> Self {
        Self {
            id,
            tabs,
            active_tab,
            width,
            state: WidgetState::Normal,
        }
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.active_tab = idx;
        }
    }

    pub fn tab_rect(&self, idx: usize, sidebar_rect: LayoutRect) -> LayoutRect {
        let item_h = 44.0;
        let gap = 4.0;
        LayoutRect::new(
            sidebar_rect.x + 8.0,
            sidebar_rect.y + 12.0 + idx as f32 * (item_h + gap),
            sidebar_rect.w - 16.0,
            item_h,
        )
    }
}

/// Modal Dialog overlay with backdrop, titlebar drag handle, window controls, and action buttons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalDialog {
    pub id: ElementId,
    pub title: String,
    pub subtitle: String,
    pub is_open: bool,
    pub confirm_label: String,
    pub cancel_label: String,
    pub width: f32,
    pub height: f32,
    pub focused_button: usize, // 0 = Confirm, 1 = Cancel
    pub state: WidgetState,
}

impl ModalDialog {
    pub const DEFAULT_WIDTH: f32 = 480.0;
    pub const DEFAULT_HEIGHT: f32 = 320.0;
    pub const TITLEBAR_HEIGHT: f32 = 44.0;
    pub const FOOTER_HEIGHT: f32 = 56.0;
    pub const CORNER_RADIUS: f32 = 16.0;

    pub fn new(id: ElementId, title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            subtitle: subtitle.into(),
            is_open: false,
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            focused_button: 0,
            state: WidgetState::Normal,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.focused_button = 0;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Full-screen dimming scrim backdrop rectangle.
    pub fn backdrop_rect(viewport_w: f32, viewport_h: f32) -> LayoutRect {
        LayoutRect::new(0.0, 0.0, viewport_w, viewport_h)
    }

    /// Backdrop dimming color: rgba(0, 0, 0, 0.60) deep liquidmorphic dimming.
    pub fn backdrop_color() -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.60]
    }

    /// Centered window frame rectangle in the viewport.
    pub fn centered_rect(&self, viewport_w: f32, viewport_h: f32) -> LayoutRect {
        let x = ((viewport_w - self.width) * 0.5).max(0.0);
        let y = ((viewport_h - self.height) * 0.5).max(0.0);
        LayoutRect::new(x, y, self.width, self.height)
    }

    /// Window titlebar rectangle.
    pub fn titlebar_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        LayoutRect::new(win_rect.x, win_rect.y, win_rect.w, Self::TITLEBAR_HEIGHT)
    }

    /// Drag handle area inside titlebar (excluding window control buttons).
    pub fn drag_handle_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        LayoutRect::new(
            win_rect.x,
            win_rect.y,
            (win_rect.w - 110.0).max(40.0),
            Self::TITLEBAR_HEIGHT,
        )
    }

    /// Close button rectangle: 24×24px at the top right of the titlebar.
    pub fn close_button_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        LayoutRect::new(
            win_rect.x + win_rect.w - 34.0,
            win_rect.y + 10.0,
            24.0,
            24.0,
        )
    }

    /// Maximize button rectangle: 24×24px adjacent to close button.
    pub fn maximize_button_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        LayoutRect::new(
            win_rect.x + win_rect.w - 62.0,
            win_rect.y + 10.0,
            24.0,
            24.0,
        )
    }

    /// Minimize button rectangle: 24×24px.
    pub fn minimize_button_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        LayoutRect::new(
            win_rect.x + win_rect.w - 90.0,
            win_rect.y + 10.0,
            24.0,
            24.0,
        )
    }

    /// Main inner content rectangle.
    pub fn content_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        let y = win_rect.y + Self::TITLEBAR_HEIGHT;
        let h = (win_rect.h - Self::TITLEBAR_HEIGHT - Self::FOOTER_HEIGHT).max(0.0);
        LayoutRect::new(
            win_rect.x + 16.0,
            y + 8.0,
            (win_rect.w - 32.0).max(0.0),
            (h - 16.0).max(0.0),
        )
    }

    /// Modal footer rectangle containing action buttons.
    pub fn footer_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        let y = win_rect.y + win_rect.h - Self::FOOTER_HEIGHT;
        LayoutRect::new(win_rect.x, y, win_rect.w, Self::FOOTER_HEIGHT)
    }

    /// Confirm (Primary CTA) button rectangle in footer.
    pub fn confirm_button_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        let btn_w = 110.0;
        let btn_h = 36.0;
        let x = win_rect.x + win_rect.w - btn_w - 20.0;
        let y = win_rect.y + win_rect.h - Self::FOOTER_HEIGHT + (Self::FOOTER_HEIGHT - btn_h) * 0.5;
        LayoutRect::new(x, y, btn_w, btn_h)
    }

    /// Cancel (Secondary) button rectangle in footer.
    pub fn cancel_button_rect(&self, win_rect: LayoutRect) -> LayoutRect {
        let btn_w = 90.0;
        let btn_h = 36.0;
        let x = win_rect.x + win_rect.w - 110.0 - 20.0 - btn_w - 12.0;
        let y = win_rect.y + win_rect.h - Self::FOOTER_HEIGHT + (Self::FOOTER_HEIGHT - btn_h) * 0.5;
        LayoutRect::new(x, y, btn_w, btn_h)
    }

    /// Traps Tab key navigation between dialog buttons.
    pub fn handle_tab(&mut self, shift: bool) {
        if shift {
            self.focused_button = if self.focused_button == 0 { 1 } else { 0 };
        } else {
            self.focused_button = (self.focused_button + 1) % 2;
        }
    }

    /// Handles Escape key: automatically dismisses/closes the dialog.
    pub fn handle_escape(&mut self) -> bool {
        if self.is_open {
            self.close();
            true
        } else {
            false
        }
    }
}

/// Toast notification with auto-dismiss duration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toast {
    pub id: ElementId,
    pub message: String,
    pub duration_sec: f32,
    pub elapsed: f32,
    pub is_dismissed: bool,
}

impl Toast {
    pub fn new(id: ElementId, message: impl Into<String>, duration_sec: f32) -> Self {
        Self {
            id,
            message: message.into(),
            duration_sec,
            elapsed: 0.0,
            is_dismissed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_dismissed {
            self.elapsed += dt;
            if self.elapsed >= self.duration_sec {
                self.is_dismissed = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_rail_and_tabs_selection() {
        let tabs = vec![
            TabItem {
                id: 0,
                title: "Profile".into(),
                badge: None,
                icon_symbol: "[USR]".into(),
            },
            TabItem {
                id: 1,
                title: "Appearance".into(),
                badge: None,
                icon_symbol: "[COL]".into(),
            },
            TabItem {
                id: 2,
                title: "Desktop".into(),
                badge: Some("14".into()),
                icon_symbol: "[DSK]".into(),
            },
        ];

        let mut rail = NavigationRail::new(ElementId::from_raw(1), tabs, 0, 220.0);
        assert_eq!(rail.active_tab, 0);

        rail.select(2);
        assert_eq!(rail.active_tab, 2);

        let r = rail.tab_rect(1, LayoutRect::new(0.0, 0.0, 220.0, 600.0));
        assert_eq!(r.w, 204.0);
        assert!(r.h >= 40.0);
    }

    #[test]
    fn test_toast_auto_dismiss() {
        let mut toast = Toast::new(ElementId::from_raw(2), "Settings Saved", 2.0);
        assert!(!toast.is_dismissed);
        toast.update(1.0);
        assert!(!toast.is_dismissed);
        toast.update(1.2);
        assert!(toast.is_dismissed);
    }

    #[test]
    fn test_modal_dialog_layout_and_focus_trapping() {
        let mut modal = ModalDialog::new(
            ElementId::from_raw(3),
            "Confirm Action",
            "Are you sure you want to proceed?",
        );
        modal.open();
        assert!(modal.is_open);

        let win_r = modal.centered_rect(1920.0, 1080.0);
        assert_eq!(win_r.w, 480.0);
        assert_eq!(win_r.h, 320.0);
        assert_eq!(win_r.x, (1920.0 - 480.0) * 0.5);

        let backdrop = ModalDialog::backdrop_rect(1920.0, 1080.0);
        assert_eq!(backdrop.w, 1920.0);
        assert_eq!(backdrop.h, 1080.0);

        let cls_r = modal.close_button_rect(win_r);
        assert_eq!(cls_r.w, 24.0);
        assert_eq!(cls_r.h, 24.0);

        let confirm_r = modal.confirm_button_rect(win_r);
        let cancel_r = modal.cancel_button_rect(win_r);
        assert!(confirm_r.x > cancel_r.x);

        // Test Tab trapping
        assert_eq!(modal.focused_button, 0);
        modal.handle_tab(false);
        assert_eq!(modal.focused_button, 1);
        modal.handle_tab(false);
        assert_eq!(modal.focused_button, 0);

        // Test Escape dismissal
        assert!(modal.handle_escape());
        assert!(!modal.is_open);
    }
}
