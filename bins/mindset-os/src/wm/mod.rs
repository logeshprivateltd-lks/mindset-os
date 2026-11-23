//! Window Manager Subsystem for MindSet OS
//!
//! Provides window bounds calculation, safe-area clamping (top bar + dock),
//! drag-to-move, edge resizing, title-bar double click, and edge snapping.

use mindset_core::{
    DesktopWidgetId, DesktopWidgetPlacement, WindowId, WindowInstance, WindowMode,
};

pub const TOP_BAR_SAFE_AREA: f32 = 44.0;
pub const DOCK_SAFE_AREA: f32 = 72.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapState {
    None,
    LeftSplit,
    RightSplit,
    Maximized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeHandle {
    Right,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct WindowInteractionState {
    pub dragging: Option<(WindowId, f32, f32)>, // (WindowId, offset_x, offset_y)
    pub dragging_widget: Option<(DesktopWidgetId, f32, f32)>, // (DesktopWidgetId, offset_x, offset_y)
    pub resizing: Option<(WindowId, ResizeHandle, f32, f32)>, // (WindowId, handle, start_w, start_h)
    pub snap_preview: SnapState,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Default for WindowInteractionState {
    fn default() -> Self {
        Self {
            dragging: None,
            dragging_widget: None,
            resizing: None,
            snap_preview: SnapState::None,
            screen_width: 1280.0,
            screen_height: 800.0,
        }
    }
}

impl WindowInteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_screen_dimensions(&mut self, width: f32, height: f32) {
        self.screen_width = width.max(800.0);
        self.screen_height = height.max(600.0);
    }

    /// Available desktop canvas height excluding top bar and floating dock safe areas
    pub fn available_height(&self) -> f32 {
        (self.screen_height - TOP_BAR_SAFE_AREA - DOCK_SAFE_AREA).max(300.0)
    }

    /// Clamp a window strictly inside the safe viewing area
    pub fn clamp_window(&self, win: &mut WindowInstance) {
        let max_w = self.screen_width;
        let max_h = self.available_height();

        win.width = win.width.clamp(400.0, max_w);
        win.height = win.height.clamp(280.0, max_h);

        win.x = win.x.clamp(0.0, (self.screen_width - win.width).max(0.0));
        win.y = win.y.clamp(
            TOP_BAR_SAFE_AREA,
            (self.screen_height - DOCK_SAFE_AREA - win.height).max(TOP_BAR_SAFE_AREA),
        );
    }

    /// Clamp a desktop widget strictly inside the safe viewing area
    pub fn clamp_widget(&self, widget: &mut DesktopWidgetPlacement) {
        widget.width = widget.width.clamp(200.0, self.screen_width);
        widget.height = widget.height.clamp(60.0, self.available_height());

        widget.x = widget.x.clamp(0.0, (self.screen_width - widget.width).max(0.0));
        widget.y = widget.y.clamp(
            TOP_BAR_SAFE_AREA,
            (self.screen_height - DOCK_SAFE_AREA - widget.height).max(TOP_BAR_SAFE_AREA),
        );
    }

    /// Snap window to a target layout mode
    pub fn snap_window(&self, win: &mut WindowInstance, snap: SnapState) {
        let av_h = self.available_height();
        match snap {
            SnapState::LeftSplit => {
                win.x = 0.0;
                win.y = TOP_BAR_SAFE_AREA;
                win.width = self.screen_width / 2.0;
                win.height = av_h;
                win.mode = WindowMode::Normal;
            }
            SnapState::RightSplit => {
                win.x = self.screen_width / 2.0;
                win.y = TOP_BAR_SAFE_AREA;
                win.width = self.screen_width / 2.0;
                win.height = av_h;
                win.mode = WindowMode::Normal;
            }
            SnapState::Maximized => {
                win.pre_max_x = win.x;
                win.pre_max_y = win.y;
                win.pre_max_width = win.width;
                win.pre_max_height = win.height;
                win.x = 0.0;
                win.y = TOP_BAR_SAFE_AREA;
                win.width = self.screen_width;
                win.height = av_h;
                win.mode = WindowMode::Maximized;
            }
            SnapState::None => {}
        }
    }

    /// Check if cursor near screen edge suggests a snap target
    pub fn check_edge_snap(&self, mouse_x: f32, mouse_y: f32) -> SnapState {
        if mouse_y <= TOP_BAR_SAFE_AREA + 10.0 {
            SnapState::Maximized
        } else if mouse_x <= 15.0 {
            SnapState::LeftSplit
        } else if mouse_x >= self.screen_width - 15.0 {
            SnapState::RightSplit
        } else {
            SnapState::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_window_within_safe_area() {
        let wm_state = WindowInteractionState::new();
        let mut win = WindowInstance::new(WindowId::StudyReader, -50.0, 10.0);

        wm_state.clamp_window(&mut win);
        assert_eq!(win.x, 0.0);
        assert_eq!(win.y, TOP_BAR_SAFE_AREA);

        // Test bottom-right out of bounds
        win.x = 2000.0;
        win.y = 1500.0;
        wm_state.clamp_window(&mut win);
        assert!(win.x <= wm_state.screen_width - win.width);
        assert!(win.y <= wm_state.screen_height - DOCK_SAFE_AREA - win.height);
    }

    #[test]
    fn test_clamp_widget_within_safe_area() {
        let wm_state = WindowInteractionState::new();
        let mut widget = DesktopWidgetPlacement::new(
            DesktopWidgetId::ScholarBriefing,
            -100.0,
            0.0,
            460.0,
            120.0,
        );

        wm_state.clamp_widget(&mut widget);
        assert_eq!(widget.x, 0.0);
        assert_eq!(widget.y, TOP_BAR_SAFE_AREA);

        widget.x = 2000.0;
        widget.y = 1200.0;
        wm_state.clamp_widget(&mut widget);
        assert!(widget.x <= wm_state.screen_width - widget.width);
        assert!(widget.y <= wm_state.screen_height - DOCK_SAFE_AREA - widget.height);
    }
}
