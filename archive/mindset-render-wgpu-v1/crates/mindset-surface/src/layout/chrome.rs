//! MindSet OS — Window Chrome Subsystem Manager (`chrome.rs`)
//! Single source of truth for window control hit-boxes (close, max, min, titlebar, resize).
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use mindset_ui::LayoutRect;

/// Window chrome button and titlebar drag hit targets, dynamically scaled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowChromeHitTargets {
    pub min_btn: LayoutRect,
    pub max_btn: LayoutRect,
    pub close_btn: LayoutRect,
    pub titlebar_drag: LayoutRect,
    pub resize_grip: LayoutRect,
}

impl WindowChromeHitTargets {
    /// Compute chrome hit targets for any window from its current LayoutRect and UI scale.
    pub fn from_rect(rect: LayoutRect, ui_scale: f32) -> Self {
        WindowChromeManager::targets(rect, ui_scale)
    }

    /// Assert that all 3 window controls (close, max, min) are distinct, non-overlapping,
    /// and meet the min 26x26 hit-box requirement.
    pub fn assert_distinct_targets(&self) -> bool {
        if self.close_btn.w < 26.0 || self.close_btn.h < 26.0 {
            return false;
        }
        if self.max_btn.w < 26.0 || self.max_btn.h < 26.0 {
            return false;
        }
        if self.min_btn.w < 26.0 || self.min_btn.h < 26.0 {
            return false;
        }
        // Check non-overlapping between close and max
        let x1 = self.close_btn.x.max(self.max_btn.x);
        let y1 = self.close_btn.y.max(self.max_btn.y);
        let x2 = (self.close_btn.x + self.close_btn.w).min(self.max_btn.x + self.max_btn.w);
        let y2 = (self.close_btn.y + self.close_btn.h).min(self.max_btn.y + self.max_btn.h);
        if x2 > x1 && y2 > y1 {
            return false;
        }
        // Check non-overlapping between max and min
        let mx1 = self.max_btn.x.max(self.min_btn.x);
        let my1 = self.max_btn.y.max(self.min_btn.y);
        let mx2 = (self.max_btn.x + self.max_btn.w).min(self.min_btn.x + self.min_btn.w);
        let my2 = (self.max_btn.y + self.max_btn.h).min(self.min_btn.y + self.min_btn.h);
        if mx2 > mx1 && my2 > my1 {
            return false;
        }
        true
    }
}

/// Manager enforcing Pattern P2: single source of truth for window chrome geometry.
pub struct WindowChromeManager;

impl WindowChromeManager {
    /// Compute chrome hit targets ensuring close is strictly distinct from maximize,
    /// with guaranteed minimum 26x26 logical hit-box.
    pub fn targets(rect: LayoutRect, ui_scale: f32) -> WindowChromeHitTargets {
        let s = ui_scale.clamp(0.80, 1.50);
        let btn_size = (26.0 * s).max(26.0); // Enforce min 26x26 logical hit box
        let gap = 4.0 * s;
        let pad_right = 6.0 * s;
        let pad_top = 4.0 * s;

        let close_x = rect.x + rect.w - pad_right - btn_size;
        let max_x = close_x - gap - btn_size;
        let min_x = max_x - gap - btn_size;
        let btn_y = rect.y + pad_top;

        let close_btn = LayoutRect::new(close_x, btn_y, btn_size, btn_size);
        let max_btn = LayoutRect::new(max_x, btn_y, btn_size, btn_size);
        let min_btn = LayoutRect::new(min_x, btn_y, btn_size, btn_size);

        let drag_w = (min_x - rect.x).max(0.0);
        let titlebar_drag = LayoutRect::new(rect.x, rect.y, drag_w, 32.0 * s);

        let grip_size = (18.0 * s).max(18.0);
        let resize_grip = LayoutRect::new(
            rect.x + rect.w - grip_size,
            rect.y + rect.h - grip_size,
            grip_size,
            grip_size,
        );

        WindowChromeHitTargets {
            min_btn,
            max_btn,
            close_btn,
            titlebar_drag,
            resize_grip,
        }
    }
}
